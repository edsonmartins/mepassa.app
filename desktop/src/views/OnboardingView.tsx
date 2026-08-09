import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import { homeDir } from '@tauri-apps/api/path'

interface OnboardingViewProps {
  localPeerId: string | null
  onUsernameRegistered?: (username: string) => void
}

export default function OnboardingView({ localPeerId, onUsernameRegistered }: OnboardingViewProps) {
  const navigate = useNavigate()
  const [showRestore, setShowRestore] = useState(false)
  const [restoreText, setRestoreText] = useState('')
  const [restoreError, setRestoreError] = useState<string | null>(null)
  const [isRestoring, setIsRestoring] = useState(false)
  const [username, setUsername] = useState('')
  const [usernameError, setUsernameError] = useState<string | null>(null)
  const [isRegistering, setIsRegistering] = useState(false)

  const handleGetStarted = async () => {
    const value = username.trim().toLowerCase()
    if (!/^[a-z0-9_]{3,20}$/.test(value)) {
      setUsernameError('Use de 3 a 20 caracteres: letras minúsculas, números ou _.')
      return
    }
    setIsRegistering(true)
    setUsernameError(null)
    try {
      await invoke('register_username', { username: value })
      onUsernameRegistered?.(value)
      navigate('/conversations')
    } catch (error) {
      setUsernameError(String(error))
    } finally {
      setIsRegistering(false)
    }
  }

  // DSK-09: restaurar backup - salva no keychain e o app REINICIA sozinho
  // com a identidade importada
  const handleRestore = async () => {
    const backup = restoreText.trim()
    if (!backup) return
    setIsRestoring(true)
    setRestoreError(null)
    try {
      const home = await homeDir()
      await invoke('import_identity_backup', {
        backup,
        dataDir: `${home}/.zaplivre`,
      })
      // import_identity_backup reinicia o app; nada mais a fazer aqui
    } catch (error) {
      setRestoreError(String(error))
      setIsRestoring(false)
    }
  }

  return (
    <div className="flex items-center justify-center h-screen bg-navy-950">
      <div className="max-w-md w-full bg-navy-900 border border-navy-800 rounded-2xl shadow-2xl p-8">
        <div className="text-center">
          {/* Logo */}
          <div className="mb-6">
            <div className="w-20 h-20 bg-brand-gradient rounded-2xl mx-auto flex items-center justify-center">
              <svg
                className="w-12 h-12 text-navy-950"
                fill="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  d="M13 2L4.5 13.5h6L9 22l8.5-11.5h-6L13 2z"
                  transform="translate(1.5,0)"
                />
              </svg>
            </div>
          </div>

          {/* Title */}
          <h1 className="text-3xl font-bold text-gray-50 mb-2">Bem-vindo ao ZapLivre</h1>
          <p className="text-gray-400 mb-6">
            Mensagens livres. Do seu jeito.
          </p>

          {/* Peer ID */}
          {localPeerId && (
            <div className="bg-navy-800 rounded-lg p-4 mb-6">
              <p className="text-xs text-gray-400 mb-1 uppercase font-semibold">Seu Peer ID</p>
              <p className="text-sm text-gray-200 font-mono break-all">{localPeerId}</p>
            </div>
          )}

          {/* Features */}
          <div className="text-left mb-8 space-y-3">
            <div className="flex items-start">
              <svg
                className="w-5 h-5 text-brand-amber mr-3 mt-0.5 flex-shrink-0"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clipRule="evenodd"
                />
              </svg>
              <div>
                <p className="font-semibold text-gray-50">80% P2P Direto</p>
                <p className="text-sm text-gray-400">Privacidade máxima, custo zero de servidor</p>
              </div>
            </div>

            <div className="flex items-start">
              <svg
                className="w-5 h-5 text-brand-amber mr-3 mt-0.5 flex-shrink-0"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clipRule="evenodd"
                />
              </svg>
              <div>
                <p className="font-semibold text-gray-50">Criptografia E2E</p>
                <p className="text-sm text-gray-400">Protocolo Signal</p>
              </div>
            </div>

            <div className="flex items-start">
              <svg
                className="w-5 h-5 text-brand-amber mr-3 mt-0.5 flex-shrink-0"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clipRule="evenodd"
                />
              </svg>
              <div>
                <p className="font-semibold text-gray-50">Sempre online</p>
                <p className="text-sm text-gray-400">Relay TURN + Store &amp; Forward</p>
              </div>
            </div>
          </div>

          <input value={username} onChange={(e) => setUsername(e.target.value)} placeholder="Escolha seu username" className="w-full bg-navy-800 border border-navy-800 rounded-lg p-3 mb-2 text-gray-100 placeholder-gray-500" />
          {usernameError && <p className="text-sm text-red-400 mb-3">{usernameError}</p>}
          <button
            onClick={handleGetStarted}
            disabled={!localPeerId || isRegistering}
            className="btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {!localPeerId ? 'Initializing...' : isRegistering ? 'Registrando...' : 'Começar'}
          </button>

          {/* Restaurar backup (DSK-09) */}
          {!showRestore ? (
            <button
              onClick={() => setShowRestore(true)}
              className="btn-secondary w-full mt-3"
            >
              Restaurar backup de identidade
            </button>
          ) : (
            <div className="mt-4 text-left">
              <p className="text-sm text-gray-600 mb-2">
                Cole o backup Base64 exportado em outro dispositivo. O app será reiniciado com a
                identidade restaurada (a identidade atual desta máquina será substituída).
              </p>
              <textarea
                value={restoreText}
                onChange={(e) => setRestoreText(e.target.value)}
                placeholder="Backup Base64..."
                className="w-full h-24 text-xs font-mono border border-gray-300 rounded-lg p-3 resize-none"
              />
              {restoreError && <p className="text-sm text-red-600 mt-2">{restoreError}</p>}
              <div className="flex gap-3 mt-3">
                <button onClick={() => setShowRestore(false)} className="btn-secondary flex-1">
                  Cancelar
                </button>
                <button
                  onClick={handleRestore}
                  disabled={!restoreText.trim() || isRestoring}
                  className="btn-primary flex-1 disabled:opacity-50"
                >
                  {isRestoring ? 'Restaurando...' : 'Restaurar e reiniciar'}
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
