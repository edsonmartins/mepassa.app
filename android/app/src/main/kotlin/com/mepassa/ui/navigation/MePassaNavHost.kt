package com.mepassa.ui.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.mepassa.ui.screens.chat.ChatScreen
import com.mepassa.ui.screens.conversations.ConversationsScreen
import com.mepassa.ui.screens.onboarding.OnboardingScreen

/**
 * Rotas de navegação do app
 */
sealed class Screen(val route: String) {
    object Onboarding : Screen("onboarding")
    object Conversations : Screen("conversations")
    object Chat : Screen("chat/{peerId}") {
        fun createRoute(peerId: String) = "chat/$peerId"
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

            ChatScreen(
                peerId = peerId,
                onNavigateBack = {
                    navController.popBackStack()
                }
            )
        }
    }
}
