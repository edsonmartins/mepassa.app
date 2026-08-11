/**
 * Mock do IPC do Tauri para testes de tela.
 *
 * Usa o mock oficial (@tauri-apps/api/mocks) com shouldMockEvents, então:
 * - `invoke('comando', args)` do app cai nos handlers configurados aqui
 * - `listen`/`emit` de '@tauri-apps/api/event' funcionam de verdade - o teste
 *   emite eventos (ex.: voip:incoming_call) e a UI reage como em produção
 *
 * Comandos não mockados explodem com erro claro - exatamente o bug de
 * "UI invocando comando inexistente" que queremos pegar.
 */
import { mockIPC } from '@tauri-apps/api/mocks'

export interface RecordedCall {
  cmd: string
  args: Record<string, unknown> | undefined
}

type CommandHandler = (args: Record<string, unknown> | undefined) => unknown

/** Defaults inofensivos para comandos de infraestrutura */
const infraDefaults: Record<string, CommandHandler> = {
  'plugin:path|resolve_directory': () => '/tmp/zaplivre-test-home',
  show_notification: () => null,
  mark_conversation_read: () => null,
  get_conversation_media: () => [],
  get_connected_peers_count: () => 1,
}

export function setupTauri(commands: Record<string, CommandHandler> = {}): {
  calls: RecordedCall[]
  callsOf: (cmd: string) => RecordedCall[]
} {
  const calls: RecordedCall[] = []

  mockIPC(
    (cmd, args) => {
      calls.push({ cmd, args: args as Record<string, unknown> | undefined })

      const handler = commands[cmd] ?? infraDefaults[cmd]
      if (handler) {
        return handler(args as Record<string, unknown> | undefined)
      }

      throw new Error(
        `Comando Tauri não mockado no teste: "${cmd}" - ou o teste esquece ` +
          `de mocká-lo, ou a UI está invocando um comando que não existe`
      )
    },
    { shouldMockEvents: true }
  )

  return {
    calls,
    callsOf: (cmd: string) => calls.filter((c) => c.cmd === cmd),
  }
}

/** Fixture de conversa no formato do comando list_conversations */
export function conversationFixture(overrides: Record<string, unknown> = {}) {
  return {
    id: '1:1:PEER_B',
    peer_id: 'PEER_B',
    display_name: 'Contato Teste',
    last_message_id: null,
    last_message_at: 1_700_000_000,
    unread_count: 0,
    ...overrides,
  }
}

/** Fixture de mensagem no formato do comando get_conversation_messages */
export function messageFixture(overrides: Record<string, unknown> = {}) {
  return {
    id: 'msg-1',
    message_id: 'msg-1',
    sender_peer_id: 'PEER_B',
    recipient_peer_id: 'PEER_A',
    content: 'Olá!',
    created_at: 1_700_000_000, // SEGUNDOS (unixepoch do SQLite)
    status: 'delivered',
    ...overrides,
  }
}

/** Fixture de mídia no formato do comando get_conversation_media */
export interface MediaItemFixture {
  id: number
  media_hash: string
  message_id: string
  media_type: string
  file_name?: string | null
  file_size?: number | null
  mime_type?: string | null
  local_path?: string | null
  thumbnail_path?: string | null
  width?: number | null
  height?: number | null
  duration_seconds?: number | null
}

export function mediaItemFixture(overrides: Record<string, unknown> = {}): MediaItemFixture {
  return {
    id: 1,
    media_hash: 'abc123hash',
    message_id: 'msg-1',
    media_type: 'image',
    file_name: 'test.jpg',
    file_size: 1024,
    mime_type: 'image/jpeg',
    local_path: '/tmp/test.jpg',
    thumbnail_path: null,
    width: 640,
    height: 480,
    duration_seconds: null,
    ...overrides,
  }
}
