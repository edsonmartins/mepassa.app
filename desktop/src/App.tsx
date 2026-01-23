import { useEffect, useState } from 'react'
import { Routes, Route, Navigate, useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import { homeDir } from '@tauri-apps/api/path'
import OnboardingView from './views/OnboardingView'
import ConversationsView from './views/ConversationsView'
import ChatView from './views/ChatView'
import CallView from './views/CallView'
import GroupListView from './views/GroupListView'
import GroupChatView from './views/GroupChatView'
import ProfileView from './views/ProfileView'

function App() {
  console.log('🔵 App component mounted')

  const [isInitialized, setIsInitialized] = useState(false)
  const [isLoading, setIsLoading] = useState(true)
  const [localPeerId, setLocalPeerId] = useState<string | null>(null)
  const [errorMessage, setErrorMessage] = useState<string>('')
  const navigate = useNavigate()

  useEffect(() => {
    console.log('🔵 useEffect running - about to call initializeApp')

    const initializeApp = async () => {
      try {
        console.log('🔵 initializeApp STARTED')
        const home = await homeDir()
        const dataDir = `${home}/.mepassa`

        console.log('🔵 Initializing MePassa with data_dir:', dataDir)

        const peerId = await invoke<string>('init_client', { dataDir })
        console.log('🔵 init_client returned peer_id:', peerId)
        setLocalPeerId(peerId)
        setIsInitialized(true)

        // Listen on default address (skip for now - runtime conflict)
        // console.log('🔵 Calling listen_on...')
        // await invoke('listen_on', { multiaddr: '/ip4/0.0.0.0/tcp/0' })

        // Bootstrap to DHT (skip for now due to runtime conflict)
        // TODO: Fix bootstrap runtime issue
        // console.log('🔵 Calling bootstrap...')
        // await invoke('bootstrap')

        console.log('✅ MePassa initialized successfully. Peer ID:', peerId)
      } catch (error) {
        console.error('❌ Failed to initialize MePassa:', error)
        const errorMsg = error instanceof Error ? error.message : String(error)
        setErrorMessage(errorMsg)
        setIsInitialized(false)
      } finally {
        setIsLoading(false)
      }
    }

    initializeApp()
  }, [])

  useEffect(() => {
    if (!isLoading && isInitialized) {
      navigate('/conversations')
    } else if (!isLoading && !isInitialized) {
      navigate('/onboarding')
    }
  }, [isLoading, isInitialized, navigate])

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-100">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-b-4 border-primary-500 mx-auto"></div>
          <p className="mt-4 text-gray-600 font-medium">Loading MePassa...</p>
          {errorMessage && (
            <div className="mt-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded">
              <p className="font-bold">Error during initialization:</p>
              <p className="text-sm mt-2">{errorMessage}</p>
            </div>
          )}
        </div>
      </div>
    )
  }

  if (errorMessage && !isInitialized) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-100">
        <div className="max-w-md w-full bg-white rounded-lg shadow-lg p-6">
          <div className="text-center">
            <div className="text-red-500 text-6xl mb-4">⚠️</div>
            <h2 className="text-2xl font-bold text-gray-900 mb-4">Initialization Failed</h2>
            <div className="p-4 bg-red-50 border border-red-200 rounded text-left">
              <p className="text-sm text-gray-700 break-all">{errorMessage}</p>
            </div>
            <button
              onClick={() => window.location.reload()}
              className="mt-6 px-4 py-2 bg-primary-500 text-white rounded hover:bg-primary-600"
            >
              Retry
            </button>
          </div>
        </div>
      </div>
    )
  }

  return (
    <Routes>
      <Route path="/onboarding" element={<OnboardingView localPeerId={localPeerId} />} />
      <Route path="/conversations" element={<ConversationsView localPeerId={localPeerId} />} />
      <Route path="/chat/:peerId" element={<ChatView localPeerId={localPeerId} />} />
      <Route path="/call/:callId/:remotePeerId" element={<CallView localPeerId={localPeerId} />} />
      <Route path="/groups" element={<GroupListView localPeerId={localPeerId} />} />
      <Route path="/group/:groupId" element={<GroupChatView />} />
      <Route path="/profile" element={<ProfileView localPeerId={localPeerId!} />} />
      <Route path="*" element={<Navigate to={isInitialized ? "/conversations" : "/onboarding"} replace />} />
    </Routes>
  )
}

export default App
