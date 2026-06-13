/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use espanso_config::config::Config;
use espanso_ui::UIRemote;

pub struct NotificationManager<'a> {
    ui_remote: &'a dyn UIRemote,
    config: &'a dyn Config,
}

impl<'a> NotificationManager<'a> {
    pub fn new(ui_remote: &'a dyn UIRemote, config: &'a dyn Config) -> Self {
        NotificationManager { ui_remote, config }
    }

    fn notify(&self, text: &str) {
        if self.config.show_notifications() {
            self.ui_remote.show_notification(text);
        }
    }

    pub fn notify_start(&self) {
        self.notify("Espanso 已启动并在后台运行中！");
    }

    pub fn notify_config_reloaded(&self, is_manual_restart: bool) {
        if is_manual_restart {
            self.notify("配置已重新加载！");
        } else {
            self.notify(
        "配置已重新加载！Espanso 会在您保存修改后自动应用。",
      );
        }
    }

    pub fn notify_keyboard_layout_reloaded(&self) {
        self.notify("键盘布局已更新！");
    }
}

impl espanso_engine::process::NotificationManager for NotificationManager<'_> {
    fn notify_status_change(&self, status: bool) {
        // Don't notify the status change outside Linux for now
        if !cfg!(target_os = "linux") {
            return;
        }

        if status {
            self.notify("Espanso 已启用！");
        } else {
            self.notify("Espanso 已禁用！");
        }
    }

    fn notify_rendering_error(&self) {
        self.notify(
            "渲染规则时发生错误，请查看系统日志以获取详细信息。",
        );
    }
}
