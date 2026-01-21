//
//  AppDelegate.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import UIKit
import UserNotifications

class AppDelegate: NSObject, UIApplicationDelegate {
    var pushManager: PushNotificationManager?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        print("📱 MePassa AppDelegate - didFinishLaunching")

        // Request push notification permissions
        pushManager?.requestAuthorization()

        // Set notification delegate
        UNUserNotificationCenter.current().delegate = pushManager

        return true
    }

    // MARK: - Push Notifications

    func application(
        _ application: UIApplication,
        didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data
    ) {
        pushManager?.didRegisterForRemoteNotifications(deviceToken: deviceToken)
    }

    func application(
        _ application: UIApplication,
        didFailToRegisterForRemoteNotificationsWithError error: Error
    ) {
        pushManager?.didFailToRegisterForRemoteNotifications(error: error)
    }
}
