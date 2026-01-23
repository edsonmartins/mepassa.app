import { QRCodeSVG } from 'qrcode.react'
import { useNavigate } from 'react-router-dom'
import { ArrowLeft, Share2, Copy, Check } from 'lucide-react'
import { useState } from 'react'

interface ProfileViewProps {
  localPeerId: string
}

export default function ProfileView({ localPeerId }: ProfileViewProps) {
  const navigate = useNavigate()
  const [copied, setCopied] = useState(false)

  const handleCopyPeerId = async () => {
    try {
      await navigator.clipboard.writeText(localPeerId)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    } catch (error) {
      console.error('Failed to copy peer ID:', error)
    }
  }

  const handleShare = async () => {
    try {
      if (navigator.share) {
        await navigator.share({
          title: 'Meu MePassa Peer ID',
          text: `Meu MePassa Peer ID: ${localPeerId}`
        })
      } else {
        await handleCopyPeerId()
      }
    } catch (error) {
      console.error('Failed to share:', error)
    }
  }

  return (
    <div className="flex flex-col h-screen bg-gray-50">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 bg-white border-b border-gray-200">
        <button
          onClick={() => navigate(-1)}
          className="p-2 hover:bg-gray-100 rounded-full transition-colors"
        >
          <ArrowLeft className="w-5 h-5 text-gray-700" />
        </button>
        <h1 className="text-lg font-semibold text-gray-900">Meu Perfil</h1>
        <div className="w-9"></div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        <div className="max-w-md mx-auto px-4 py-8 space-y-8">
          {/* QR Code Section */}
          <div className="bg-white rounded-2xl p-8 shadow-sm border border-gray-100">
            <h2 className="text-xl font-bold text-gray-900 mb-6 text-center">
              Meu QR Code
            </h2>

            {/* QR Code */}
            <div className="flex justify-center mb-6">
              <div className="p-6 bg-white rounded-xl shadow-lg">
                <QRCodeSVG
                  value={localPeerId}
                  size={220}
                  level="M"
                  includeMargin={true}
                />
              </div>
            </div>

            {/* Peer ID Display */}
            <div className="space-y-2">
              <p className="text-xs font-medium text-gray-500 text-center">
                Peer ID
              </p>
              <div className="relative">
                <div className="px-4 py-3 bg-gray-50 rounded-lg font-mono text-xs text-gray-700 break-all text-center border border-gray-200">
                  {localPeerId}
                </div>
                <button
                  onClick={handleCopyPeerId}
                  className="absolute right-2 top-1/2 -translate-y-1/2 p-2 hover:bg-gray-200 rounded-lg transition-colors"
                  title="Copiar Peer ID"
                >
                  {copied ? (
                    <Check className="w-4 h-4 text-green-600" />
                  ) : (
                    <Copy className="w-4 h-4 text-gray-600" />
                  )}
                </button>
              </div>
              {copied && (
                <p className="text-xs text-green-600 text-center animate-fade-in">
                  ✓ Peer ID copiado!
                </p>
              )}
            </div>
          </div>

          {/* Action Buttons */}
          <div className="space-y-3">
            <button
              onClick={handleShare}
              className="w-full flex items-center justify-center gap-2 px-6 py-3 bg-primary-500 text-white font-semibold rounded-xl hover:bg-primary-600 transition-colors shadow-sm"
            >
              <Share2 className="w-5 h-5" />
              Compartilhar
            </button>

            <button
              onClick={handleCopyPeerId}
              className="w-full flex items-center justify-center gap-2 px-6 py-3 bg-gray-100 text-gray-700 font-semibold rounded-xl hover:bg-gray-200 transition-colors"
            >
              <Copy className="w-5 h-5" />
              {copied ? 'Copiado!' : 'Copiar Peer ID'}
            </button>
          </div>

          {/* Info Section */}
          <div className="bg-blue-50 border border-blue-100 rounded-xl p-4">
            <h3 className="font-semibold text-blue-900 mb-2">
              Como usar o QR Code
            </h3>
            <ul className="text-sm text-blue-800 space-y-1">
              <li>• Compartilhe este QR code com seus contatos</li>
              <li>• Eles podem escaneá-lo no app mobile</li>
              <li>• Ou copie e envie seu Peer ID diretamente</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  )
}
