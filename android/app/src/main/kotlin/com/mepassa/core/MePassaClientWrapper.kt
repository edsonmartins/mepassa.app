package com.mepassa.core

import android.content.Context
import android.util.Log
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.withContext
import uniffi.mepassa.*
import java.io.File

/**
 * Wrapper Singleton para MePassaClient do UniFFI
 *
 * Fornece:
 * - Inicialização lazy do client
 * - API coroutine-friendly
 * - Estado observável (Flows)
 * - Gerenciamento de ciclo de vida
 */
object MePassaClientWrapper {
    private const val TAG = "MePassaClientWrapper"

    private var client: MePassaClient? = null
    private val _isInitialized = MutableStateFlow(false)
    val isInitialized: StateFlow<Boolean> = _isInitialized.asStateFlow()

    private val _localPeerId = MutableStateFlow<String?>(null)
    val localPeerId: StateFlow<String?> = _localPeerId.asStateFlow()

    /**
     * Inicializa o MePassaClient
     *
     * @param context Application context
     * @return true se inicializado com sucesso
     */
    suspend fun initialize(context: Context): Boolean = withContext(Dispatchers.IO) {
        if (client != null) {
            Log.w(TAG, "Client already initialized")
            return@withContext true
        }

        try {
            // Diretório de dados do app
            val dataDir = File(context.filesDir, "mepassa_data").apply {
                if (!exists()) {
                    mkdirs()
                }
            }

            Log.i(TAG, "Initializing MePassaClient with dataDir: ${dataDir.absolutePath}")

            // Criar client via UniFFI
            client = MePassaClient(dataDir.absolutePath)

            // Obter peer ID local
            val peerId = client!!.localPeerId()
            _localPeerId.value = peerId
            _isInitialized.value = true

            Log.i(TAG, "Client initialized successfully. PeerId: $peerId")
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to initialize client", e)
            _isInitialized.value = false
            false
        }
    }

    /**
     * Obtém o client (deve ser inicializado primeiro)
     */
    fun getClient(): MePassaClient {
        return client ?: throw IllegalStateException("Client not initialized. Call initialize() first.")
    }

    /**
     * Verifica se o client está inicializado
     */
    fun isClientReady(): Boolean = client != null

    /**
     * Lista todas as conversas
     */
    suspend fun listConversations(): List<FfiConversation> = withContext(Dispatchers.IO) {
        try {
            getClient().listConversations()
        } catch (e: Exception) {
            Log.e(TAG, "Failed to list conversations", e)
            emptyList()
        }
    }

    /**
     * Busca mensagens de uma conversa
     */
    suspend fun getConversationMessages(
        peerId: String,
        limit: UInt? = null,
        offset: UInt? = null
    ): List<FfiMessage> = withContext(Dispatchers.IO) {
        try {
            getClient().getConversationMessages(peerId, limit, offset)
        } catch (e: Exception) {
            Log.e(TAG, "Failed to get conversation messages", e)
            emptyList()
        }
    }

    /**
     * Envia mensagem de texto
     */
    suspend fun sendTextMessage(toPeerId: String, content: String): Result<String> =
        withContext(Dispatchers.IO) {
            try {
                val messageId = getClient().sendTextMessage(toPeerId, content)
                Result.success(messageId)
            } catch (e: Exception) {
                Log.e(TAG, "Failed to send text message", e)
                Result.failure(e)
            }
        }

    /**
     * Marca conversa como lida
     */
    suspend fun markConversationRead(peerId: String): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().markConversationRead(peerId)
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to mark conversation as read", e)
            false
        }
    }

    /**
     * Busca mensagens (full-text search)
     */
    suspend fun searchMessages(query: String, limit: UInt? = null): List<FfiMessage> =
        withContext(Dispatchers.IO) {
            try {
                getClient().searchMessages(query, limit)
            } catch (e: Exception) {
                Log.e(TAG, "Failed to search messages", e)
                emptyList()
            }
        }

    /**
     * Conecta a um peer específico
     */
    suspend fun connectToPeer(peerId: String, multiaddr: String): Boolean =
        withContext(Dispatchers.IO) {
            try {
                getClient().connectToPeer(peerId, multiaddr)
                true
            } catch (e: Exception) {
                Log.e(TAG, "Failed to connect to peer", e)
                false
            }
        }

    /**
     * Inicia escuta em um endereço
     */
    suspend fun listenOn(multiaddr: String): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().listenOn(multiaddr)
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to listen on address", e)
            false
        }
    }

    /**
     * Faz bootstrap (conecta aos bootstrap nodes)
     */
    suspend fun bootstrap(): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().bootstrap()
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to bootstrap", e)
            false
        }
    }

    /**
     * Obtém contagem de peers conectados
     */
    suspend fun getConnectedPeersCount(): UInt = withContext(Dispatchers.IO) {
        try {
            getClient().connectedPeersCount()
        } catch (e: Exception) {
            Log.e(TAG, "Failed to get connected peers count", e)
            0u
        }
    }

    // ========== VoIP Methods ==========

    /**
     * Inicia uma chamada de voz para um peer
     *
     * @param toPeerId ID do peer de destino
     * @return Result com call_id se sucesso, ou Exception se falha
     */
    suspend fun startCall(toPeerId: String): Result<String> = withContext(Dispatchers.IO) {
        try {
            val callId = getClient().startCall(toPeerId)
            Log.i(TAG, "Call started successfully: $callId")
            Result.success(callId)
        } catch (e: Exception) {
            Log.e(TAG, "Failed to start call", e)
            Result.failure(e)
        }
    }

    /**
     * Aceita uma chamada recebida
     *
     * @param callId ID da chamada
     * @return true se sucesso
     */
    suspend fun acceptCall(callId: String): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().acceptCall(callId)
            Log.i(TAG, "Call accepted: $callId")
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to accept call", e)
            false
        }
    }

    /**
     * Rejeita uma chamada recebida
     *
     * @param callId ID da chamada
     * @param reason Motivo da rejeição (opcional)
     * @return true se sucesso
     */
    suspend fun rejectCall(callId: String, reason: String? = null): Boolean =
        withContext(Dispatchers.IO) {
            try {
                getClient().rejectCall(callId, reason)
                Log.i(TAG, "Call rejected: $callId")
                true
            } catch (e: Exception) {
                Log.e(TAG, "Failed to reject call", e)
                false
            }
        }

    /**
     * Encerra uma chamada ativa
     *
     * @param callId ID da chamada
     * @return true se sucesso
     */
    suspend fun hangupCall(callId: String): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().hangupCall(callId)
            Log.i(TAG, "Call hung up: $callId")
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to hangup call", e)
            false
        }
    }

    /**
     * Alterna mute do microfone
     *
     * @param callId ID da chamada
     * @return true se sucesso
     */
    suspend fun toggleMute(callId: String): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().toggleMute(callId)
            Log.i(TAG, "Mute toggled for call: $callId")
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to toggle mute", e)
            false
        }
    }

    /**
     * Alterna speakerphone
     *
     * @param callId ID da chamada
     * @return true se sucesso
     */
    suspend fun toggleSpeakerphone(callId: String): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().toggleSpeakerphone(callId)
            Log.i(TAG, "Speakerphone toggled for call: $callId")
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to toggle speakerphone", e)
            false
        }
    }

    // ========== Group Methods (FASE 15) ==========

    /**
     * Cria um novo grupo
     *
     * @param name Nome do grupo
     * @param description Descrição do grupo (opcional)
     * @return FfiGroup criado
     */
    suspend fun createGroup(name: String, description: String?): FfiGroup =
        withContext(Dispatchers.IO) {
            try {
                val group = getClient().createGroup(name, description)
                Log.i(TAG, "Group created successfully: ${group.id}")
                group
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create group", e)
                throw e
            }
        }

    /**
     * Entra em um grupo existente
     *
     * @param groupId ID do grupo
     * @param groupName Nome do grupo
     */
    suspend fun joinGroup(groupId: String, groupName: String): Boolean =
        withContext(Dispatchers.IO) {
            try {
                getClient().joinGroup(groupId, groupName)
                Log.i(TAG, "Joined group successfully: $groupId")
                true
            } catch (e: Exception) {
                Log.e(TAG, "Failed to join group", e)
                false
            }
        }

    /**
     * Sai de um grupo
     *
     * @param groupId ID do grupo
     */
    suspend fun leaveGroup(groupId: String): Boolean = withContext(Dispatchers.IO) {
        try {
            getClient().leaveGroup(groupId)
            Log.i(TAG, "Left group successfully: $groupId")
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to leave group", e)
            false
        }
    }

    /**
     * Adiciona um membro ao grupo (apenas admin)
     *
     * @param groupId ID do grupo
     * @param peerId ID do peer a adicionar
     */
    suspend fun addGroupMember(groupId: String, peerId: String): Boolean =
        withContext(Dispatchers.IO) {
            try {
                getClient().addGroupMember(groupId, peerId)
                Log.i(TAG, "Added member to group $groupId: $peerId")
                true
            } catch (e: Exception) {
                Log.e(TAG, "Failed to add group member", e)
                throw e
            }
        }

    /**
     * Remove um membro do grupo (apenas admin)
     *
     * @param groupId ID do grupo
     * @param peerId ID do peer a remover
     */
    suspend fun removeGroupMember(groupId: String, peerId: String): Boolean =
        withContext(Dispatchers.IO) {
            try {
                getClient().removeGroupMember(groupId, peerId)
                Log.i(TAG, "Removed member from group $groupId: $peerId")
                true
            } catch (e: Exception) {
                Log.e(TAG, "Failed to remove group member", e)
                false
            }
        }

    /**
     * Lista todos os grupos do usuário
     *
     * @return Lista de grupos
     */
    suspend fun getGroups(): List<FfiGroup> = withContext(Dispatchers.IO) {
        try {
            getClient().getGroups()
        } catch (e: Exception) {
            Log.e(TAG, "Failed to get groups", e)
            emptyList()
        }
    }

    // ========== Video Methods (FASE 14) ==========

    /**
     * Enable video for an active call
     *
     * @param callId Call identifier
     * @param codec Video codec to use (H264, VP8, VP9)
     */
    suspend fun enableVideo(callId: String, codec: FfiVideoCodec = FfiVideoCodec.H264) = withContext(Dispatchers.IO) {
        try {
            getClient().enableVideo(callId, codec)
            Log.i(TAG, "Video enabled for call: $callId with codec: $codec")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to enable video for call: $callId", e)
            throw e
        }
    }

    /**
     * Disable video for an active call
     *
     * @param callId Call identifier
     */
    suspend fun disableVideo(callId: String) = withContext(Dispatchers.IO) {
        try {
            getClient().disableVideo(callId)
            Log.i(TAG, "Video disabled for call: $callId")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to disable video for call: $callId", e)
            throw e
        }
    }

    /**
     * Send video frame to remote peer
     *
     * @param callId Call identifier
     * @param frameData Raw frame data (pre-encoded H.264/VP8 NALUs)
     * @param width Frame width in pixels
     * @param height Frame height in pixels
     */
    suspend fun sendVideoFrame(
        callId: String,
        frameData: ByteArray,
        width: UInt,
        height: UInt
    ) = withContext(Dispatchers.IO) {
        try {
            getClient().sendVideoFrame(callId, frameData.toList(), width, height)
        } catch (e: Exception) {
            Log.e(TAG, "Failed to send video frame for call: $callId", e)
            // Don't throw - frame drops are acceptable
        }
    }

    /**
     * Switch camera (front/back) during video call
     *
     * @param callId Call identifier
     */
    suspend fun switchCamera(callId: String) = withContext(Dispatchers.IO) {
        try {
            getClient().switchCamera(callId)
            Log.i(TAG, "Camera switched for call: $callId")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to switch camera for call: $callId", e)
            throw e
        }
    }

    /**
     * Shutdown do client (chame no onDestroy da Application)
     */
    fun shutdown() {
        try {
            client = null
            _isInitialized.value = false
            _localPeerId.value = null
            Log.i(TAG, "Client shutdown completed")
        } catch (e: Exception) {
            Log.e(TAG, "Error during shutdown", e)
        }
    }
}
