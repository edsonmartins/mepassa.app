# Guia de Contribuição - MePassa

Obrigado por considerar contribuir com o MePassa! 🎉

## 📋 Código de Conduta

Este projeto segue o [Código de Conduta](CODE_OF_CONDUCT.md). Ao participar, você concorda em manter um ambiente respeitoso e acolhedor.

## 🚀 Como Contribuir

### 1. Issues

**Reportar Bugs:**
- Use o template de issue "Bug Report"
- Descreva o problema claramente
- Inclua passos para reproduzir
- Ambiente (OS, versão do app, etc)

**Sugerir Features:**
- Use o template "Feature Request"
- Explique o caso de uso
- Considere alternativas

**Boas práticas:**
- 🔍 Procure por issues duplicadas antes de criar
- 📝 Seja claro e conciso
- 🏷️ Use labels apropriadas

### 2. Pull Requests

**Antes de começar:**
1. Comente na issue que você vai trabalhar nela
2. Fork o repositório
3. Crie uma branch: `git checkout -b feature/sua-feature`

**Durante o desenvolvimento:**
- Siga o [style guide](#style-guide)
- Escreva commits claros ([Conventional Commits](https://www.conventionalcommits.org/))
- Mantenha PRs focados (uma feature por PR)

**Ao finalizar:**
1. Rode os testes: `cargo test --workspace`
2. Rode o linter: `cargo clippy -- -D warnings`
3. Formate o código: `cargo fmt`
4. Push para seu fork: `git push origin feature/sua-feature`
5. Abra PR com descrição detalhada

**Template de PR:**
```markdown
## Descrição
[Descreva a mudança]

## Tipo de mudança
- [ ] Bug fix
- [ ] Nova feature
- [ ] Breaking change
- [ ] Documentação

## Como foi testado?
[Descreva os testes]

## Checklist
- [ ] Testes passam localmente
- [ ] Código formatado (`cargo fmt`)
- [ ] Sem warnings (`cargo clippy`)
- [ ] Documentação atualizada
```

### 3. Áreas de Contribuição

**🦀 Core (Rust)**
- **Fácil:** Documentação, testes unitários, exemplos
- **Médio:** Implementar módulos específicos (storage, protocol)
- **Difícil:** Networking P2P, criptografia, WebRTC

**📱 Mobile**
- **Android:** Kotlin + Jetpack Compose
- **iOS:** Swift + SwiftUI

**🖥️ Desktop**
- **Tauri:** Rust backend + React frontend

**🎨 Design**
- UI/UX mockups (Figma)
- Ícones e assets
- Guias de estilo

**📝 Documentação**
- Tutoriais
- Guias de arquitetura
- Tradução (i18n)

## 🔧 Setup de Desenvolvimento

### Pré-requisitos
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Ferramentas
cargo install cargo-watch
cargo install cargo-edit
```

### Clone e Build
```bash
git clone https://github.com/integralltech/mepassa.git
cd mepassa

# Build core
cd core
cargo build

# Rodar testes
cargo test
```

### Desenvolvimento Local
```bash
# Watch mode (recompila ao salvar)
cargo watch -x check -x test

# Rodar exemplo
cargo run --example simple_chat
```

## 📏 Style Guide

### Rust

**Formatação:**
```bash
# Formatar código
cargo fmt

# Verificar formatação
cargo fmt -- --check
```

**Linting:**
```bash
# Rodar clippy
cargo clippy -- -D warnings
```

**Convenções:**
- Use `snake_case` para funções e variáveis
- Use `PascalCase` para structs e enums
- Máximo 100 caracteres por linha
- Documente funções públicas com `///`
- Use `Result<T>` para funções que podem falhar

**Exemplo:**
```rust
/// Envia mensagem de texto para destinatário
///
/// # Argumentos
/// * `recipient` - Peer ID do destinatário
/// * `text` - Conteúdo da mensagem
///
/// # Retorna
/// ID da mensagem enviada
///
/// # Erros
/// Retorna erro se destinatário não encontrado ou rede falhar
pub async fn send_text(&mut self, recipient: &str, text: String) -> Result<String> {
    // ...
}
```

### Commits

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
tipo(escopo): descrição curta

Descrição detalhada (opcional)

Refs: #123
```

**Tipos:**
- `feat`: Nova feature
- `fix`: Bug fix
- `docs`: Documentação
- `style`: Formatação
- `refactor`: Refatoração
- `test`: Testes
- `chore`: Manutenção

**Exemplos:**
```bash
feat(crypto): implementar Signal Protocol E2E
fix(network): corrigir NAT traversal em alguns roteadores
docs(readme): atualizar instruções de build
```

## 🧪 Testes

### Rodar testes
```bash
# Todos os testes
cargo test --workspace

# Testes de um módulo específico
cargo test --package mepassa-core --lib identity

# Testes com output
cargo test -- --nocapture
```

### Escrever testes
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = Keypair::generate();
        assert!(keypair.peer_id().starts_with("mepassa_"));
    }

    #[tokio::test]
    async fn test_send_message() {
        let mut client = create_test_client().await;
        let result = client.send_text("bob", "test".to_string()).await;
        assert!(result.is_ok());
    }
}
```

## 📚 Recursos

- [Rust Book](https://doc.rust-lang.org/book/)
- [libp2p Tutorial](https://docs.libp2p.io/tutorials/)
- [Signal Protocol Docs](https://signal.org/docs/)
- [Tauri Guide](https://tauri.app/v2/guides/)

## 🏷️ Labels

- `good first issue`: Bom para iniciantes
- `help wanted`: Precisamos de ajuda
- `bug`: Algo não funciona
- `enhancement`: Nova feature ou melhoria
- `documentation`: Melhorias na documentação
- `priority: high`: Alta prioridade

## ❓ Perguntas?

- **Discord:** *(em breve)*
- **Matrix:** *(em breve)*
- **Email:** contato@integralltech.com.br

---

**Obrigado por contribuir! 🙏**
