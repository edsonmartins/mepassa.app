# Auditoria de dependência: libsignal-protocol-syft 0.85.3-beta.5

**Data:** 2026-08-08
**Contexto:** Fase D (PLANO_HOMOLOGACAO.md, item D4)

## O que é

- Crate Rust que implementa o Signal Protocol (X3DH + Double Ratchet) usada em
  `core/src/crypto/signal.rs` para criptografia E2E 1:1.
- **Fork** do `libsignal-protocol` oficial com sufixo `-syft` e versão
  `0.85.3-beta.5` (pré-lançamento).

## Como está declarada

```toml
# core/Cargo.toml
libsignal-protocol-syft = "0.85.3-beta.5"
```

O `Cargo.lock` trava a versão (checksum `3e970a79f9941dbd090a49d4a97dc2c13f93c67af1c2cf87833b8a9f8519b579`).
Dependências diretas: `libsignal-core-syft 0.85.3-beta.5`, `signal-crypto-syft`,
`spqr-syft`, `libcrux-ml-kem`, etc.

## Riscos

| Risco | Severidade | Mitigação |
|---|---|---|
| Fork não-oficial com versão beta | Alta | Pin exata no `Cargo.lock` (feito); revisar diffs vs upstream ao atualizar |
| Sem auditoria de terceiros da cadeia `-syft` | Alta | Fase D exige revisão manual do código usado; considerar lock + SBOM (F4) |
| Atualização para nova beta pode quebrar ABI de storage (session records) | Média | `core/src/crypto/signal.rs` versiona o armazenamento; testar upgrade com dados persistidos |
| Sem plano de upgrade para versão estável | Média | Monitorar upstream `libsignal-protocol` e migrar quando estável |

## Veredito

**Aceitável para beta fechado** (base técnica madura), com as seguintes
condições:

1. **Pin exato** (manter no `Cargo.lock`; não usar `*` nem caret que resolva
   para outra beta). Já atendido.
2. **Revisão do que é consumido**: o `signal.rs` usa apenas as primitivas de
   sessão (SessionCipher, PreKeyBundle, session records). Nenhuma feature
   experimental (ex.: PQXDH/libcrux-ml-kem) está ativada por config — apenas as
   APis padrão de session. Confirmar que não há chamadas a módulos
   experimentais novos.
3. **Plano de upgrade**: registrar issue para acompanhar versões estáveis do
   `libsignal-protocol` (público) e migrar o fork quando houver release estável
   compatível. Critério: testes `crypto::signal` + `message_integration` +
   `reliability` verdes após bump.
4. **SBOM/scan**: incluir esta crate no scan de vulnerabilidades das imagens
   (F4 do plano) para capturar CVEs anunciadas.

## Checklist de acompanhamento

- [ ] Registrar issue de upgrade para `libsignal-protocol` estável
- [ ] Revisar diffs `-syft` vs upstream antes de cada bump
- [ ] Adicionar ao SBOM das imagens (F4)
- [ ] Testar upgrade com sessões persistidas (ABI de storage)
