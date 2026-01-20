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
