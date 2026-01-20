package com.mepassa

import android.Manifest
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.core.content.ContextCompat
import androidx.lifecycle.lifecycleScope
import com.mepassa.core.MePassaClientWrapper
import com.mepassa.service.MePassaService
import com.mepassa.ui.navigation.MePassaNavHost
import com.mepassa.ui.theme.MePassaTheme
import kotlinx.coroutines.launch

/**
 * MainActivity - Ponto de entrada do app
 *
 * Responsabilidades:
 * - Inicializar MePassaClient
 * - Solicitar permissões necessárias
 * - Iniciar MePassaService
 * - Configurar navegação Compose
 */
class MainActivity : ComponentActivity() {

    companion object {
        private const val TAG = "MainActivity"
    }

    // Launcher para solicitar permissão de notificação (Android 13+)
    private val notificationPermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestPermission()
    ) { isGranted ->
        if (isGranted) {
            Log.i(TAG, "Notification permission granted")
            startService()
        } else {
            Log.w(TAG, "Notification permission denied")
            // Ainda assim iniciar service (notificação não aparecerá)
            startService()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        Log.i(TAG, "MainActivity created")

        // Inicializar MePassaClient
        lifecycleScope.launch {
            val success = MePassaClientWrapper.initialize(applicationContext)
            if (!success) {
                Log.e(TAG, "Failed to initialize MePassaClient")
                // TODO: Mostrar erro para usuário
            } else {
                Log.i(TAG, "MePassaClient initialized successfully")
            }
        }

        // Solicitar permissões e iniciar service
        requestPermissionsAndStartService()

        // Setup UI
        setContent {
            MePassaTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    MePassaApp()
                }
            }
        }
    }

    /**
     * Solicita permissões necessárias e inicia service
     */
    private fun requestPermissionsAndStartService() {
        // Android 13+ requer permissão explícita para notificações
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            when {
                ContextCompat.checkSelfPermission(
                    this,
                    Manifest.permission.POST_NOTIFICATIONS
                ) == PackageManager.PERMISSION_GRANTED -> {
                    Log.i(TAG, "Notification permission already granted")
                    startService()
                }
                shouldShowRequestPermissionRationale(Manifest.permission.POST_NOTIFICATIONS) -> {
                    // TODO: Mostrar explicação ao usuário
                    Log.i(TAG, "Should show notification permission rationale")
                    notificationPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
                }
                else -> {
                    Log.i(TAG, "Requesting notification permission")
                    notificationPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
                }
            }
        } else {
            // Versões anteriores não precisam permissão explícita
            startService()
        }
    }

    /**
     * Inicia MePassaService
     */
    private fun startService() {
        Log.i(TAG, "Starting MePassaService")
        MePassaService.start(this)
    }

    override fun onDestroy() {
        super.onDestroy()
        Log.i(TAG, "MainActivity destroyed")
        // NÃO parar o service aqui, pois queremos que continue em background
    }
}

/**
 * Composable principal do app
 */
@Composable
fun MePassaApp() {
    val isInitialized by MePassaClientWrapper.isInitialized.collectAsState()

    MePassaNavHost(isClientInitialized = isInitialized)
}
