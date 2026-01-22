import { useEffect, useState, useRef } from 'react'
import { useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'

interface Conversation {
  peer_id: string
  last_message: string | null
  last_message_timestamp: number
  unread_count: number
}

interface ConversationsViewProps {
  localPeerId: string | null
}

export default function ConversationsView({ localPeerId }: ConversationsViewProps) {
  const [conversations, setConversations] = useState<Conversation[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [showNewChatDialog, setShowNewChatDialog] = useState(false)
  const [newPeerId, setNewPeerId] = useState('')
  const [peerCount, setPeerCount] = useState(0)
  const navigate = useNavigate()
  const previousConversations = useRef<Conversation[]>([])

  useEffect(() => {
    loadConversations()
    loadPeerCount()

    // Auto-refresh every 5 seconds
    const interval = setInterval(() => {
      loadConversations()
      loadPeerCount()
    }, 5000)

    return () => clearInterval(interval)
  }, [])

  const loadConversations = async () => {
    try {
      const convs = await invoke<Conversation[]>('list_conversations')

      // Detect new messages
      if (previousConversations.current.length > 0) {
        for (const newConv of convs) {
          const oldConv = previousConversations.current.find(c => c.peer_id === newConv.peer_id)

          // New conversation or new unread messages
          if (!oldConv || (newConv.unread_count > 0 && newConv.unread_count > oldConv.unread_count)) {
            // Show notification
            try {
              await invoke('show_notification', {
                title: 'Nova mensagem',
                body: newConv.last_message || `Mensagem de ${newConv.peer_id.substring(0, 8)}...`
              })
            } catch (error) {
              console.error('Failed to show notification:', error)
            }
          }
        }
      }

      // Update state
      previousConversations.current = convs
      setConversations(convs)
    } catch (error) {
      console.error('Failed to load conversations:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const loadPeerCount = async () => {
    try {
      const count = await invoke<number>('get_connected_peers_count')
      setPeerCount(count)
    } catch (error) {
      console.error('Failed to load peer count:', error)
    }
  }

  const handleNewChat = async () => {
    if (!newPeerId.trim()) return

    try {
      // Navigate to chat view
      navigate(`/chat/${newPeerId}`)
      setShowNewChatDialog(false)
      setNewPeerId('')
    } catch (error) {
      console.error('Failed to start new chat:', error)
    }
  }

  const formatTimestamp = (timestamp: number): string => {
    const date = new Date(timestamp * 1000)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    const diffHours = Math.floor(diffMs / 3600000)
    const diffDays = Math.floor(diffMs / 86400000)

    if (diffMins < 1) return 'Just now'
    if (diffMins < 60) return `${diffMins}m ago`
    if (diffHours < 24) return `${diffHours}h ago`
    if (diffDays < 7) return `${diffDays}d ago`
    return date.toLocaleDateString()
  }

  return (
    <div className="flex flex-col h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">MePassa</h1>
            {localPeerId && (
              <p className="text-xs text-gray-500 font-mono truncate max-w-xs">
                {localPeerId}
              </p>
            )}
          </div>
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2 text-sm text-gray-600">
              <div className={`w-2 h-2 rounded-full ${peerCount > 0 ? 'bg-green-500' : 'bg-gray-400'}`}></div>
              <span>{peerCount} peers</span>
            </div>
            <button
              onClick={() => navigate('/groups')}
              className="btn-secondary text-sm"
            >
              Groups
            </button>
            <button
              onClick={() => setShowNewChatDialog(true)}
              className="btn-primary text-sm"
            >
              + New Chat
            </button>
          </div>
        </div>
      </div>

      {/* Conversations List */}
      <div className="flex-1 overflow-y-auto">
        {isLoading ? (
          <div className="flex items-center justify-center h-full">
            <div className="animate-spin rounded-full h-12 w-12 border-b-4 border-primary-500"></div>
          </div>
        ) : conversations.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center px-6">
            <svg
              className="w-24 h-24 text-gray-300 mb-4"
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
            <h2 className="text-xl font-semibold text-gray-900 mb-2">No conversations yet</h2>
            <p className="text-gray-600 mb-6">
              Start a new chat by clicking the "New Chat" button
            </p>
            <button
              onClick={() => setShowNewChatDialog(true)}
              className="btn-primary"
            >
              Start Your First Chat
            </button>
          </div>
        ) : (
          <div className="divide-y divide-gray-200">
            {conversations.map((conv) => (
              <div
                key={conv.peer_id}
                onClick={() => navigate(`/chat/${conv.peer_id}`)}
                className="px-6 py-4 hover:bg-gray-50 cursor-pointer transition-colors"
              >
                <div className="flex items-center justify-between">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between mb-1">
                      <p className="text-sm font-semibold text-gray-900 truncate">
                        {conv.peer_id.substring(0, 16)}...
                      </p>
                      <p className="text-xs text-gray-500 ml-2">
                        {formatTimestamp(conv.last_message_timestamp)}
                      </p>
                    </div>
                    <p className="text-sm text-gray-600 truncate">
                      {conv.last_message || 'No messages yet'}
                    </p>
                  </div>
                  {conv.unread_count > 0 && (
                    <div className="ml-4">
                      <span className="inline-flex items-center justify-center w-6 h-6 text-xs font-bold text-white bg-primary-500 rounded-full">
                        {conv.unread_count}
                      </span>
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* New Chat Dialog */}
      {showNewChatDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl p-6 w-full max-w-md mx-4">
            <h2 className="text-xl font-bold text-gray-900 mb-4">New Chat</h2>
            <input
              type="text"
              value={newPeerId}
              onChange={(e) => setNewPeerId(e.target.value)}
              placeholder="Enter peer ID..."
              className="input-base mb-4"
              autoFocus
              onKeyPress={(e) => e.key === 'Enter' && handleNewChat()}
            />
            <div className="flex space-x-3">
              <button
                onClick={() => setShowNewChatDialog(false)}
                className="btn-secondary flex-1"
              >
                Cancel
              </button>
              <button
                onClick={handleNewChat}
                disabled={!newPeerId.trim()}
                className="btn-primary flex-1 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Start Chat
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
