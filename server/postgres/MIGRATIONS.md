# Postgres Schema Migration Strategy

**Data:** 2026-08-07
**Status:** Planejado — adotar `sqlx::migrate!`

## Estado atual (problema)

- O schema é definido **apenas** em `server/postgres/init.sql`, montado em
  `/docker-entrypoint-initdb.d/` no `docker-compose.yml`.
- O Docker executa esse diretório **somente quando o volume está vazio**
  (primeira inicialização). Qualquer alteração no `init.sql` depois disso
  **não tem efeito** em bancos existentes.
- Os serviços (`store`, `identity`, `push`) usam `sqlx` **sem** `sqlx::migrate`
  — não há versionamento nem aplicação automática de evoluções de schema.

## Estratégia recomendada: sqlx migrations

1. **O dono do schema é o `message-store`.** As tabelas de negócio
   (`offline_messages`, `push_tokens`, `user_presence`, `message_stats`,
   `usernames`) pertencem ao `zaplivre-store`. Migrações vivem em
   `server/store/migrations/`.

2. **Adotar `sqlx::migrate!`** no `zaplivre-store`:
   ```rust
   // server/store/src/main.rs
   sqlx::migrate!("./migrations")
       .run(&pool)
       .await
       .expect("failed to run migrations");
   ```
   Executado logo após criar o pool, antes de iniciar o HTTP server.

3. **Seed das migrações:** criar `migrations/0001_init.sql` com o conteúdo do
   `init.sql` (tabelas + funções + índices). O `init.sql` do compose passa a ser
   apenas um "bootstrap rápido para dev", e o contrato real de schema vira a
   pasta de migrações.

4. **Processo para evoluir o schema:**
   - Criar `migrations/XXXX_descricao.sql` com `ALTER`/`CREATE` incrementais.
   - Nunca editar migrações já aplicadas (imutáveis).
   - Rodar `cargo sqlx prepare` (com `SQLX_OFFLINE=false`) se usar `query!`
     compile-time.
   - Testar em dev: `docker compose down && docker compose up -d postgres`
     (volume zerado) **e** upgrade real (volume existente).

5. **Migrações para produção (C8 no plano de homologação):**
   - `sqlx::migrate!` roda idempotente no boot do `store` → evolução automática
     no deploy (comum em deploys rolling).
   - Backup antes de migrar: `pg_dump` (ver C8 - backup/restore).
   - Rotação/rollback: nunca reverter DDL por código; restaurar do dump se
     necessário.

## Notas

- `identity/schema.sql` é espelho da tabela `usernames` para deploy standalone.
  Após a adoção das migrações no `store`, avaliar mover o `identity` para o
  mesmo mecanismo (cada serviço dono do próprio schema) ou centralizar tudo no
  `store` com uma única pasta de migrações.
- `pg_cron` (cleanup de mensagens expiradas) é opcional e ignorado se a
  extensão não estiver disponível; não é bloqueio para migrações.
