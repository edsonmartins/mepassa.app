import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { useNavigate } from 'react-router-dom'
import '../styles/IncomingCallModal.css'

interface IncomingCallModalProps {
  callId: string
  callerPeerId: string
  onClose: () => void
}

export default function IncomingCallModal({
  callId,
  callerPeerId,
  onClose,
}: IncomingCallModalProps) {
  const navigate = useNavigate()
  const [isAccepting, setIsAccepting] = useState(false)
  const [isRejecting, setIsRejecting] = useState(false)

  // Escape key to reject
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        handleReject()
      }
    }

    window.addEventListener('keydown', handleEscape)
    return () => window.removeEventListener('keydown', handleEscape)
  }, [])

  const handleAccept = async () => {
    setIsAccepting(true)

    try {
      await invoke('accept_call', { callId })
      // Navigate to call view
      navigate(`/call/${callId}/${callerPeerId}`)
      onClose()
    } catch (error) {
      console.error('Failed to accept call:', error)
      setIsAccepting(false)
    }
  }

  const handleReject = async () => {
    setIsRejecting(true)

    try {
      await invoke('reject_call', { callId, reason: 'User declined' })
      onClose()
    } catch (error) {
      console.error('Failed to reject call:', error)
      onClose() // Close anyway
    }
  }

  return (
    <div className="incoming-call-overlay" onClick={handleReject}>
      <div className="incoming-call-modal" onClick={(e) => e.stopPropagation()}>
        {/* Animated Avatar */}
        <div className="incoming-call-avatar">
          <svg
            width="160"
            height="160"
            viewBox="0 0 120 120"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <circle cx="60" cy="60" r="60" fill="#3b82f6" />
            <path
              d="M60 60c11.046 0 20-8.954 20-20s-8.954-20-20-20-20 8.954-20 20 8.954 20 20 20zm0 10c-13.314 0-40 6.686-40 20v10h80v-10c0-13.314-26.686-20-40-20z"
              fill="white"
            />
          </svg>
        </div>

        {/* Caller Info */}
        <h2 className="caller-name">
          {callerPeerId.substring(0, 16)}...
        </h2>

        <p className="call-type">Chamada de voz recebida</p>

        {/* Phone Icon */}
        <div className="phone-icon">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z" />
          </svg>
        </div>

        {/* Action Buttons */}
        <div className="incoming-call-actions">
          {/* Reject Button */}
          <button
            onClick={handleReject}
            disabled={isRejecting || isAccepting}
            className="action-btn reject-btn"
          >
            <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 9c-1.6 0-3.15.25-4.6.72v3.1c0 .39-.23.74-.56.9-.98.49-1.87 1.12-2.66 1.85-.18.18-.43.28-.7.28-.28 0-.53-.11-.71-.29L.29 13.08c-.18-.17-.29-.42-.29-.70 0-.28.11-.53.29-.71C3.34 8.78 7.46 7 12 7s8.66 1.78 11.71 4.67c.18.18.29.43.29.71 0 .28-.11.53-.29.71l-2.48 2.48c-.18.18-.43.29-.71.29-.27 0-.52-.11-.7-.28-.79-.74-1.69-1.36-2.67-1.85-.33-.16-.56-.5-.56-.9v-3.1C15.15 9.25 13.6 9 12 9z" />
            </svg>
            <span>Recusar</span>
          </button>

          {/* Accept Button */}
          <button
            onClick={handleAccept}
            disabled={isAccepting || isRejecting}
            className="action-btn accept-btn"
          >
            <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor">
              <path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z" />
            </svg>
            <span>Atender</span>
          </button>
        </div>

        <p className="hint-text">Pressione ESC para recusar</p>
      </div>
    </div>
  )
}
