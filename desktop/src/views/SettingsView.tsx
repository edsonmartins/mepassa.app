import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { ArrowLeft, RefreshCw } from 'lucide-react'

interface SettingsViewProps {
  localPeerId: string | null
}

/**
 * Configurações (desktop). Espelha o SettingsView do iOS: toggles de
 * notificação/privacidade (estado local, ainda não persistido — mesma
 * limitação documentada no Maestro) + identidade (peer ID, backup, limpar
 * estado). A navegação de volta usa o header com botão.
 */
export default function SettingsView({ localPeerId }: SettingsViewProps) {
  const [notificationsEnabled, setNotificationsEnabled] = useState(true)
  const [soundEnabled, setSoundEnabled] = useState(true)
  const [vibrationEnabled, setVibrationEnabled] = useState(true)
  const [readReceiptsEnabled, setReadReceiptsEnabled] = useState(true)
  const [lastSeenEnabled, setLastSeenEnabled] = useState(true)
  const navigate = useNavigate()

  useEffect(() => {
    // Navegação de volta com botão físico do Tauri (histórico)
    const unlistenPromise = import('@tauri-apps/api/core').then(() =>
      import('@tauri-apps/api/event').then(({ listen }) =>
        listen('tauri://back-requested', () => navigate('/conversations'))
      )
    )
    return () => {
      unlistenPromise.then((unlisten) => unlisten()).catch(() => {})
    }
  }, [navigate])

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-200">
        <div className="flex items-center gap-2">
          <button
            onClick={() => navigate('/conversations')}
            className="p-2 hover:bg-gray-100 rounded-full"
            title="Voltar"
          >
            <ArrowLeft className="w-5 h-5" />
          </button>
          <h1 className="text-lg font-bold text-gray-900">Configurações</h1>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4 max-w-2xl mx-auto w-full space-y-6">
        {/* Identidade */}
        <section>
          <h2 className="text-sm font-semibold text-gray-500 uppercase mb-2">Identidade</h2>
          <div className="bg-white rounded-xl border border-gray-200 p-4">
            <p className="text-sm text-gray-600 mb-1">Meu peer ID</p>
            <p className="font-mono text-xs break-all text-gray-900">
              {localPeerId ?? '—'}
            </p>
          </div>
        </section>

        {/* Notificações */}
        <section>
          <h2 className="text-sm font-semibold text-gray-500 uppercase mb-2">Notificações</h2>
          <div className="bg-white rounded-xl border border-gray-200 divide-y divide-gray-100">
            <ToggleRow
              label="Ativar notificações"
              checked={notificationsEnabled}
              onChange={setNotificationsEnabled}
            />
            <ToggleRow
              label="Som"
              checked={soundEnabled}
              disabled={!notificationsEnabled}
              onChange={setSoundEnabled}
            />
            <ToggleRow
              label="Vibração"
              checked={vibrationEnabled}
              disabled={!notificationsEnabled}
              onChange={setVibrationEnabled}
            />
          </div>
        </section>

        {/* Privacidade */}
        <section>
          <h2 className="text-sm font-semibold text-gray-500 uppercase mb-2">Privacidade</h2>
          <div className="bg-white rounded-xl border border-gray-200 divide-y divide-gray-100">
            <ToggleRow
              label="Confirmações de leitura"
              checked={readReceiptsEnabled}
              onChange={setReadReceiptsEnabled}
            />
            <ToggleRow
              label="Última visualização"
              checked={lastSeenEnabled}
              onChange={setLastSeenEnabled}
            />
          </div>
        </section>

        {/* Dados */}
        <section>
          <h2 className="text-sm font-semibold text-gray-500 uppercase mb-2">Dados</h2>
          <div className="bg-white rounded-xl border border-gray-200 divide-y divide-gray-100">
            <button
              onClick={() => navigate('/conversations', { state: { openBackup: true } })}
              className="flex items-center gap-2 px-4 py-3 text-sm text-gray-800 hover:bg-gray-50 w-full text-left"
            >
              <RefreshCw className="w-4 h-4" />
              Exportar backup da identidade
            </button>
          </div>
        </section>

        <p className="text-xs text-gray-400">
          ZapLivre Desktop — os toggles são locais (persistência de preferências é um item em aberto).
        </p>
      </div>
    </div>
  )
}

interface ToggleRowProps {
  label: string
  checked: boolean
  disabled?: boolean
  onChange: (value: boolean) => void
}

function ToggleRow({ label, checked, disabled, onChange }: ToggleRowProps) {
  return (
    <div className="flex items-center justify-between px-4 py-3">
      <span className="text-sm text-gray-800">{label}</span>
      <button
        role="switch"
        aria-checked={checked}
        disabled={disabled}
        onClick={() => onChange(!checked)}
        className={`relative w-11 h-6 rounded-full transition-colors ${
          disabled
            ? 'bg-gray-200 cursor-not-allowed'
            : checked
              ? 'bg-green-500'
              : 'bg-gray-300'
        }`}
        title={label}
      >
        <span
          className={`absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform ${
            checked ? 'translate-x-5' : ''
          }`}
        />
      </button>
    </div>
  )
}
