import { useEffect, useState, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'

interface Message {
  id: string
  from_peer_id: string
  to_peer_id: string
  content: string
  timestamp: number
  is_read: boolean
}

interface ChatViewProps {
  localPeerId: string | null
}

export default function ChatView({ localPeerId }: ChatViewProps) {
  const { peerId } = useParams<{ peerId: string }>()
  const navigate = useNavigate()
  const [messages, setMessages] = useState<Message[]>([])
  const [newMessage, setNewMessage] = useState('')
  const [isSending, setIsSending] = useState(false)
  const [isLoading, setIsLoading] = useState(true)
  const messagesEndRef = useRef<HTMLDivElement>(null)
  const previousMessageCount = useRef<number>(0)

  useEffect(() => {
    if (!peerId) return

    loadMessages()
    markAsRead()

    // Auto-refresh every 2 seconds
    const interval = setInterval(loadMessages, 2000)

    return () => clearInterval(interval)
  }, [peerId])

  useEffect(() => {
    scrollToBottom()
  }, [messages])

  const loadMessages = async () => {
    if (!peerId) return

    try {
      const msgs = await invoke<Message[]>('get_conversation_messages', {
        peerId,
        limit: 100,
        offset: 0,
      })

      // Detect new received messages
      if (previousMessageCount.current > 0 && msgs.length > previousMessageCount.current) {
        const newMessages = msgs.slice(previousMessageCount.current)
        for (const msg of newMessages) {
          // Only notify for received messages (not sent by me)
          if (msg.from_peer_id !== localPeerId) {
            try {
              await invoke('show_notification', {
                title: `Nova mensagem de ${msg.from_peer_id.substring(0, 8)}...`,
                body: msg.content.substring(0, 100)
              })
            } catch (error) {
              console.error('Failed to show notification:', error)
            }
          }
        }
      }

      previousMessageCount.current = msgs.length
      setMessages(msgs)
    } catch (error) {
      console.error('Failed to load messages:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const markAsRead = async () => {
    if (!peerId) return

    try {
      await invoke('mark_conversation_read', { peerId })
    } catch (error) {
      console.error('Failed to mark as read:', error)
    }
  }

  const handleSend = async () => {
    if (!newMessage.trim() || !peerId || isSending) return

    setIsSending(true)

    try {
      await invoke('send_text_message', {
        toPeerId: peerId,
        content: newMessage.trim(),
      })

      setNewMessage('')
      // Reload messages to show sent message
      await loadMessages()
    } catch (error) {
      console.error('Failed to send message:', error)
      alert('Failed to send message. Please try again.')
    } finally {
      setIsSending(false)
    }
  }

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }

  const formatTime = (timestamp: number): string => {
    const date = new Date(timestamp * 1000)
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  }

  const isSentByMe = (msg: Message): boolean => {
    return msg.from_peer_id === localPeerId
  }

  return (
    <div className="flex flex-col h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="flex items-center">
          <button
            onClick={() => navigate('/conversations')}
            className="mr-4 text-gray-600 hover:text-gray-900"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </button>
          <div className="flex-1">
            <h2 className="text-lg font-semibold text-gray-900">
              {peerId?.substring(0, 16)}...
            </h2>
            <p className="text-xs text-gray-500 font-mono truncate max-w-md">{peerId}</p>
          </div>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4">
        {isLoading ? (
          <div className="flex items-center justify-center h-full">
            <div className="animate-spin rounded-full h-12 w-12 border-b-4 border-primary-500"></div>
          </div>
        ) : messages.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center">
            <svg
              className="w-20 h-20 text-gray-300 mb-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
              />
            </svg>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">No messages yet</h3>
            <p className="text-gray-600">Send a message to start the conversation</p>
          </div>
        ) : (
          <>
            {messages.map((msg) => (
              <div
                key={msg.id}
                className={`flex ${isSentByMe(msg) ? 'justify-end' : 'justify-start'}`}
              >
                <div className={isSentByMe(msg) ? 'message-sent' : 'message-received'}>
                  <p className="whitespace-pre-wrap">{msg.content}</p>
                  <p
                    className={`text-xs mt-1 ${
                      isSentByMe(msg) ? 'text-primary-100' : 'text-gray-500'
                    }`}
                  >
                    {formatTime(msg.timestamp)}
                  </p>
                </div>
              </div>
            ))}
            <div ref={messagesEndRef} />
          </>
        )}
      </div>

      {/* Input */}
      <div className="bg-white border-t border-gray-200 px-6 py-4">
        <div className="flex items-center space-x-3">
          <input
            type="text"
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && !e.shiftKey && handleSend()}
            placeholder="Type a message..."
            className="input-base flex-1"
            disabled={isSending}
          />
          <button
            onClick={handleSend}
            disabled={!newMessage.trim() || isSending}
            className="btn-primary px-6 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSending ? (
              <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
            ) : (
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                />
              </svg>
            )}
          </button>
        </div>
      </div>
    </div>
  )
}
