import { useEffect, useState } from 'react'
import { Routes, Route, Navigate, useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import { homeDir } from '@tauri-apps/api/path'
import OnboardingView from './views/OnboardingView'
import ConversationsView from './views/ConversationsView'
import ChatView from './views/ChatView'

function App() {
  const [isInitialized, setIsInitialized] = useState(false)
  const [isLoading, setIsLoading] = useState(true)
  const [localPeerId, setLocalPeerId] = useState<string | null>(null)
  const navigate = useNavigate()

  useEffect(() => {
    const initializeApp = async () => {
      try {
        const home = await homeDir()
        const dataDir = `${home}/.mepassa`

        console.log('Initializing MePassa with data_dir:', dataDir)

        const peerId = await invoke<string>('init_client', { dataDir })
        setLocalPeerId(peerId)
        setIsInitialized(true)

        // Listen on default address
        await invoke('listen_on', { multiaddr: '/ip4/0.0.0.0/tcp/0' })

        // Bootstrap to DHT
        await invoke('bootstrap')

        console.log('MePassa initialized successfully. Peer ID:', peerId)
      } catch (error) {
        console.error('Failed to initialize MePassa:', error)
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
        </div>
      </div>
    )
  }

  return (
    <Routes>
      <Route path="/onboarding" element={<OnboardingView localPeerId={localPeerId} />} />
      <Route path="/conversations" element={<ConversationsView localPeerId={localPeerId} />} />
      <Route path="/chat/:peerId" element={<ChatView localPeerId={localPeerId} />} />
      <Route path="*" element={<Navigate to={isInitialized ? "/conversations" : "/onboarding"} replace />} />
    </Routes>
  )
}

export default App
