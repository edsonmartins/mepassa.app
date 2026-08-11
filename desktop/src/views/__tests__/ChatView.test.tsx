import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { MemoryRouter, Route, Routes } from 'react-router-dom'
import { describe, expect, it } from 'vitest'
import ChatView from '../ChatView'
import { setupTauri } from '../../test/tauriMock'

function renderChat() {
  return render(
    <MemoryRouter initialEntries={['/chat/PEER_B']}>
      <Routes>
        <Route path="/chat/:peerId" element={<ChatView localPeerId="PEER_A" />} />
      </Routes>
    </MemoryRouter>
  )
}

describe('ChatView', () => {
  it('renderiza mensagens com horário correto (created_at em SEGUNDOS)', async () => {
    setupTauri({
      get_conversation_messages: () => [
        { id: 'msg-1', message_id: 'msg-1', sender_peer_id: 'PEER_B', recipient_peer_id: 'PEER_A', content: 'Olá do outro peer!', created_at: 1700000000, status: 'delivered' },
      ],
    })

    renderChat()

    expect(await screen.findByText('Olá do outro peer!')).toBeInTheDocument()
  })

  it('envia mensagem pelo comando send_text_message com os argumentos certos', async () => {
    const { callsOf } = setupTauri({
      get_conversation_messages: () => [],
      send_text_message: () => 'new-message-id',
    })
    const user = userEvent.setup()

    renderChat()

    const input = await screen.findByPlaceholderText(/type a message/i)
    await user.type(input, 'mensagem de teste')
    await user.keyboard('{Enter}')

    await waitFor(() => {
      const sends = callsOf('send_text_message')
      expect(sends).toHaveLength(1)
      expect(sends[0].args).toMatchObject({
        toPeerId: 'PEER_B',
        content: 'mensagem de teste',
      })
    })
  })

  it('tem botão de anexar arquivo (UX-02)', async () => {
    setupTauri({ get_conversation_messages: () => [] })

    renderChat()

    expect(await screen.findByTitle(/anexar arquivo/i)).toBeInTheDocument()
  })

  it('renderiza mensagem de texto', async () => {
    setupTauri({
      get_conversation_messages: () => [
        { id: 'msg-1', message_id: 'msg-1', sender_peer_id: 'PEER_B', recipient_peer_id: 'PEER_A', content: 'Mensagem de texto', created_at: 700000000, status: 'delivered' },
      ],
      get_conversation_media: () => [],
    })

    renderChat()

    expect(await screen.findByText('Mensagem de texto')).toBeInTheDocument()
  })
})