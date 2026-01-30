/**
 * Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
 * Copyright (C) 2025  Miku Push! Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

using System;
using Microsoft.UI.Xaml;
using Windows.ApplicationModel.Activation;
using Microsoft.Windows.AppLifecycle;
using System.Diagnostics;
using Windows.ApplicationModel.DataTransfer;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.Linq;

// To learn more about WinUI, the WinUI project structure,
// and more about our project templates, see: http://aka.ms/winui-project-info.

namespace mikupush_helpers
{
    /// <summary>
    /// Provides application-specific behavior to supplement the default Application class.
    /// </summary>
    public partial class App : Application
    {
        /// <summary>
        /// Initializes the singleton application object.  This is the first line of authored code
        /// executed, and as such is the logical equivalent of main() or WinMain().
        /// </summary>
        public App()
        {
            InitializeComponent();
        }

        /// <summary>
        /// Invoked when the application is launched.
        /// </summary>
        /// <param name="args">Details about the launch request and process.</param>
        protected override void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args)
        {
            Debug.WriteLine("Cheking Sharing target is activated");
            AppInstance? currentAppInstance = AppInstance.GetCurrent();
            AppActivationArguments? activatedEvent = currentAppInstance.GetActivatedEventArgs();

            if (activatedEvent != null && activatedEvent.Data is ShareTargetActivatedEventArgs shareArgs)
            {
                _ = OnShareTargetActivated(shareArgs);
            }

            currentAppInstance.Activated += OnActivated;
        }

        private void OnActivated(object? sender, AppActivationArguments args)
        {
            if (args.Kind == ExtendedActivationKind.ShareTarget && args.Data is ShareTargetActivatedEventArgs shareData)
            {
                _ = OnShareTargetActivated(shareData);
            }
        }

        private async Task OnShareTargetActivated(ShareTargetActivatedEventArgs args)
        {
            DataPackageView data = args.ShareOperation.Data;
            Debug.WriteLine($"available formats: {string.Join(",", data.AvailableFormats)}");

            try
            {
                List<string> filePaths = new List<string> { };

                if (data.Contains(StandardDataFormats.StorageItems))
                {
                    var items = await data.GetStorageItemsAsync();

                    foreach (var item in items)
                    {
                        Debug.WriteLine($"Shared file path: {item.Path}");
                        filePaths.Add(item.Path);
                    }
                }

                Debug.WriteLine($"Collected shared file paths: {filePaths}");

                if (filePaths.Count > 0)
                {
                    RequestFileUpload(filePaths.ToArray());
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Failed to emit share request: {ex.Message}");
            }

            Exit();
        }

        private void RequestFileUpload(string[] paths)
        {
            string arguments = string.Join(" ", paths.Select(path => $"\"{path}\""));
            Debug.WriteLine($"Requesting file upload for: {paths}");
            Debug.WriteLine($"Launching mikupush.exe with arguments: {arguments}");

            try
            {
                Process.Start(new ProcessStartInfo { 
                    FileName = "mikupush.exe",
                    Arguments = arguments,
                    UseShellExecute = true 
                });
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Failed to open deep-link: {ex.Message}");
            }
        }
    }
}
