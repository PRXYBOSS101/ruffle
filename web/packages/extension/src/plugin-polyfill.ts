// This file is compiled and then injected into content.ts's compiled form.

import {
    installPlugin,
    FLASH_PLUGIN,
} from "ruffle-core/pkg/plugin-polyfill.js";

installPlugin(FLASH_PLUGIN);
