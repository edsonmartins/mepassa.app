import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest'
import { VoiceMessageBubble } from '../VoiceMessageBubble'

// Mock HTMLAudioElement
const mockPlay = vi.fn().mockResolvedValue(undefined)
const mockPause = vi.fn()

beforeEach(() => {
  Element.prototype.scrollTo = vi.fn()
  // Mock HTMLMediaElement.prototype.play and pause
  HTMLMediaElement.prototype.play = mockPlay
  HTMLMediaElement.prototype.pause = mockPause
})

afterEach(() => {
  mockPlay.mockRestore()
  mockPause.mockRestore()
})

describe('VoiceMessageBubble', () => {
  it('renders voice message with duration', () => {
    render(<VoiceMessageBubble audioFilePath="/test/audio.mp3" durationSeconds={42} />)

    expect(screen.getByText('0:42')).toBeInTheDocument()
  })

  it('shows play button initially', () => {
    render(<VoiceMessageBubble audioFilePath="/test/audio.mp3" durationSeconds={30} />)

    const playButton = screen.getByRole('button', { name: /play/i })
    expect(playButton).toBeInTheDocument()
  })

  it('renders with correct colors for own message', () => {
    const { container } = render(
      <VoiceMessageBubble audioFilePath="/test/audio.mp3" durationSeconds={10} isOwnMessage />
    )

    const bubble = container.querySelector('div')
    expect(bubble?.classList.contains('bg-primary')).toBe(true)
  })

  it('renders with correct colors for received message', () => {
    const { container } = render(
      <VoiceMessageBubble audioFilePath="/test/audio.mp3" durationSeconds={10} isOwnMessage={false} />
    )

    const bubble = container.querySelector('div')
    expect(bubble?.classList.contains('bg-gray-100')).toBe(true)
  })

  it('has an audio element with correct src', () => {
    render(<VoiceMessageBubble audioFilePath="/test/audio.mp3" durationSeconds={30} />)

    const audioElement = document.querySelector('audio')
    expect(audioElement).toBeTruthy()
    expect(audioElement?.getAttribute('src')).toBe('/test/audio.mp3')
    expect(audioElement?.getAttribute('preload')).toBe('metadata')
  })
})