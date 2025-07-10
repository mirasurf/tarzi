// Firefox preferences for headless browser automation
user_pref("browser.shell.checkDefaultBrowser", false);
user_pref("browser.startup.page", 0);
user_pref("browser.startup.homepage_override.mstone", "ignore");
user_pref("browser.startup.homepage", "about:blank");
user_pref("startup.homepage_welcome_url", "");
user_pref("startup.homepage_welcome_url.additional", "");
user_pref("browser.startup.firstrunSkipsHomepage", true);

// Disable various Firefox features for automation
user_pref("browser.tabs.warnOnClose", false);
user_pref("browser.tabs.warnOnCloseOtherTabs", false);
user_pref("browser.tabs.warnOnOpen", false);
user_pref("browser.sessionstore.resume_from_crash", false);
user_pref("browser.crashReports.unsubmittedCheck.enabled", false);

// Security and privacy settings for automation
user_pref("security.tls.insecure_fallback_hosts", "");
user_pref("security.warn_viewing_mixed", false);
user_pref("security.warn_viewing_mixed.show_once", false);
user_pref("security.warn_submit_insecure", false);
user_pref("security.warn_submit_insecure.show_once", false);

// Disable automatic updates
user_pref("app.update.enabled", false);
user_pref("app.update.auto", false);
user_pref("app.update.mode", 0);
user_pref("app.update.service.enabled", false);

// Performance settings
user_pref("dom.max_script_run_time", 0);
user_pref("dom.max_chrome_script_run_time", 0);
user_pref("browser.dom.window.dump.enabled", false);
user_pref("devtools.console.stdout.chrome", false);

// Disable extensions and add-ons prompts
user_pref("extensions.blocklist.enabled", false);
user_pref("extensions.checkCompatibility", false);
user_pref("extensions.checkUpdateSecurity", false);
user_pref("extensions.update.enabled", false);
user_pref("extensions.update.autoUpdateDefault", false);

// Network settings
user_pref("network.http.phishy-userpass-length", 255);
user_pref("network.automatic-ntlm-auth.trusted-uris", "");

// Media settings
user_pref("media.volume_scale", "0.01");
user_pref("media.gmp-manager.updateEnabled", false);

// Geolocation and permissions
user_pref("geo.enabled", false);
user_pref("geo.provider.use_corelocation", false);
user_pref("geo.prompt.testing", false);
user_pref("geo.prompt.testing.allow", false);

// Downloads
user_pref("browser.download.manager.showWhenStarting", false);
user_pref("browser.download.dir", "/app/browser-data/downloads");
user_pref("browser.helperApps.neverAsk.saveToDisk", "application/octet-stream,text/csv,text/xml,application/xml,text/plain,text/css,text/tab-separated-values,text/html,application/xhtml+xml,application/zip");

// Logging
user_pref("browser.dom.window.dump.enabled", false);
user_pref("devtools.console.stdout.chrome", false);
user_pref("toolkit.startup.max_resumed_crashes", -1);