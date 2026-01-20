package com.mepassa.ui.navigation

import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.mepassa.core.MePassaClientWrapper
import com.mepassa.ui.screens.call.CallScreen
import com.mepassa.ui.screens.call.IncomingCallScreen
import com.mepassa.ui.screens.chat.ChatScreen
import com.mepassa.ui.screens.conversations.ConversationsScreen
import com.mepassa.ui.screens.onboarding.OnboardingScreen
import kotlinx.coroutines.launch

/**
 * Rotas de navegação do app
 */
sealed class Screen(val route: String) {
    object Onboarding : Screen("onboarding")
    object Conversations : Screen("conversations")
    object Chat : Screen("chat/{peerId}") {
        fun createRoute(peerId: String) = "chat/$peerId"
    }
    object IncomingCall : Screen("incoming_call/{callId}/{callerPeerId}") {
        fun createRoute(callId: String, callerPeerId: String) = "incoming_call/$callId/$callerPeerId"
    }
    object ActiveCall : Screen("active_call/{callId}/{remotePeerId}") {
        fun createRoute(callId: String, remotePeerId: String) = "active_call/$callId/$remotePeerId"
    }
}

/**
 * NavHost principal do app
 *
 * Gerencia navegação entre:
 * - Onboarding (primeira vez)
 * - Conversations (lista de conversas)
 * - Chat (conversa específica)
 */
@Composable
fun MePassaNavHost(
    isClientInitialized: Boolean,
    navController: NavHostController = rememberNavController()
) {
    // Determina tela inicial baseado no estado do client
    val startDestination = if (isClientInitialized) {
        Screen.Conversations.route
    } else {
        Screen.Onboarding.route
    }

    NavHost(
        navController = navController,
        startDestination = startDestination
    ) {
        // Onboarding
        composable(Screen.Onboarding.route) {
            OnboardingScreen(
                onOnboardingComplete = {
                    // Navegar para Conversations e remover Onboarding da pilha
                    navController.navigate(Screen.Conversations.route) {
                        popUpTo(Screen.Onboarding.route) { inclusive = true }
                    }
                }
            )
        }

        // Lista de conversas
        composable(Screen.Conversations.route) {
            ConversationsScreen(
                onConversationClick = { peerId ->
                    navController.navigate(Screen.Chat.createRoute(peerId))
                }
            )
        }

        // Chat (conversa específica)
        composable(
            route = Screen.Chat.route,
            arguments = listOf(
                navArgument("peerId") { type = NavType.StringType }
            )
        ) { backStackEntry ->
            val peerId = backStackEntry.arguments?.getString("peerId") ?: return@composable
            val scope = rememberCoroutineScope()

            ChatScreen(
                peerId = peerId,
                onNavigateBack = {
                    navController.popBackStack()
                },
                onStartCall = {
                    // Inicia chamada e navega para ActiveCall
                    scope.launch {
                        val result = MePassaClientWrapper.startCall(peerId)
                        result.onSuccess { callId ->
                            navController.navigate(Screen.ActiveCall.createRoute(callId, peerId))
                        }.onFailure { error ->
                            // TODO: Mostrar erro (Snackbar ou Toast)
                            android.util.Log.e("ChatScreen", "Failed to start call: ${error.message}")
                        }
                    }
                }
            )
        }

        // Incoming Call (chamada recebida - fullscreen)
        composable(
            route = Screen.IncomingCall.route,
            arguments = listOf(
                navArgument("callId") { type = NavType.StringType },
                navArgument("callerPeerId") { type = NavType.StringType }
            )
        ) { backStackEntry ->
            val callId = backStackEntry.arguments?.getString("callId") ?: return@composable
            val callerPeerId = backStackEntry.arguments?.getString("callerPeerId") ?: return@composable

            IncomingCallScreen(
                callId = callId,
                callerPeerId = callerPeerId,
                onAccept = {
                    // Navegar para ActiveCall e remover IncomingCall da pilha
                    navController.navigate(Screen.ActiveCall.createRoute(callId, callerPeerId)) {
                        popUpTo(Screen.IncomingCall.route) { inclusive = true }
                    }
                },
                onReject = {
                    // Voltar para tela anterior
                    navController.popBackStack()
                }
            )
        }

        // Active Call (chamada ativa)
        composable(
            route = Screen.ActiveCall.route,
            arguments = listOf(
                navArgument("callId") { type = NavType.StringType },
                navArgument("remotePeerId") { type = NavType.StringType }
            )
        ) { backStackEntry ->
            val callId = backStackEntry.arguments?.getString("callId") ?: return@composable
            val remotePeerId = backStackEntry.arguments?.getString("remotePeerId") ?: return@composable

            CallScreen(
                callId = callId,
                remotePeerId = remotePeerId,
                onCallEnded = {
                    // Voltar para Conversations
                    navController.popBackStack(Screen.Conversations.route, inclusive = false)
                }
            )
        }
    }
}
