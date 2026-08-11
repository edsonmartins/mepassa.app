import { useState, useRef, useEffect } from 'react'
import { Play, Pause } from 'lucide-react'

interface VoiceMessageBubbleProps {
  audioFilePath: string
  durationSeconds?: number
  isOwnMessage?: boolean
  className?: string
}

export function VoiceMessageBubble({
  audioFilePath,
  durationSeconds,
  isOwnMessage = false,
  className = '',
}: VoiceMessageBubbleProps) {
  const [isPlaying, setIsPlaying] = useState(false)
  const [currentTime, setCurrentTime] = useState(0)
  const [duration, setDuration] = useState(durationSeconds || 0)

  const audioRef = useRef<HTMLAudioElement>(null)

  useEffect(() => {
    const audio = audioRef.current
    if (!audio) return

    const updateTime = () => setCurrentTime(audio.currentTime)
    const setAudioDuration = () => setDuration(audio.duration || 0)

    audio.addEventListener('timeupdate', updateTime)
    audio.addEventListener('loadedmetadata', setAudioDuration)

    return () => {
      audio.removeEventListener('timeupdate', updateTime)
      audio.removeEventListener('loadedmetadata', setAudioDuration)
    }
  }, [])

  useEffect(() => {
    if (isPlaying && audioRef.current) {
      audioRef.current.play().catch(console.error)
    } else if (audioRef.current) {
      audioRef.current.pause()
    }
  }, [isPlaying])

  useEffect(() => {
    if (!isPlaying && audioRef.current) {
      setCurrentTime(0)
    }
  }, [isPlaying])

  const togglePlay = () => {
    setIsPlaying(!isPlaying)
  }

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  const progress = duration > 0 ? currentTime / duration : 0

  const baseClasses = 'flex items-center gap-3 rounded-2xl px-4 py-2'
  const colorClasses = isOwnMessage
    ? 'bg-primary text-primary-foreground'
    : 'bg-gray-100 text-gray-900 dark:bg-gray-800 dark:text-gray-100'

  return (
    <>
      <audio ref={audioRef} src={audioFilePath} preload="metadata" />
      <div
        className={`${baseClasses} ${colorClasses} ${className}`}
      >
        <button
          onClick={togglePlay}
          className={`flex-shrink-0 rounded-full p-1.5 transition-colors ${
            isOwnMessage
              ? 'bg-white/20 hover:bg-white/30'
              : 'bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600'
          }`}
          aria-label={isPlaying ? 'Pause' : 'Play'}
        >
          {isPlaying ? (
            <Pause className="h-4 w-4" />
          ) : (
            <Play className="h-4 w-4" />
          )}
        </button>

        <div className="flex-1">
          <div className="text-xs font-medium">{formatTime(duration)}</div>
          <div className="h-1 bg-white/20 dark:bg-gray-700 rounded-full overflow-hidden">
            <div
              className="h-full bg-white dark:bg-gray-300 rounded-full transition-all"
              style={{ width: `${progress * 100}%` }}
            />
          </div>
        </div>
      </div>
    </>
  )
}