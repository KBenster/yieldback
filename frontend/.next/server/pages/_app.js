/*
 * ATTENTION: An "eval-source-map" devtool has been used.
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file with attached SourceMaps in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(() => {
var exports = {};
exports.id = "pages/_app";
exports.ids = ["pages/_app"];
exports.modules = {

/***/ "(pages-dir-node)/./public/fonts/dm-sans.css":
/*!**********************************!*\
  !*** ./public/fonts/dm-sans.css ***!
  \**********************************/
/***/ (() => {



/***/ }),

/***/ "(pages-dir-node)/./src/contexts/settings.tsx":
/*!***********************************!*\
  !*** ./src/contexts/settings.tsx ***!
  \***********************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   SettingsProvider: () => (/* binding */ SettingsProvider),\n/* harmony export */   useNetwork: () => (/* binding */ useNetwork),\n/* harmony export */   useNotifications: () => (/* binding */ useNotifications),\n/* harmony export */   usePositionDefaults: () => (/* binding */ usePositionDefaults),\n/* harmony export */   useSettings: () => (/* binding */ useSettings),\n/* harmony export */   useTheme: () => (/* binding */ useTheme),\n/* harmony export */   useTransactionSettings: () => (/* binding */ useTransactionSettings)\n/* harmony export */ });\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! react/jsx-dev-runtime */ \"react/jsx-dev-runtime\");\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! react */ \"react\");\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_1__);\n\n\nconst defaultSettings = {\n    // Theme\n    theme: 'light',\n    // Network\n    network: 'testnet',\n    customRpcUrl: 'https://soroban-testnet.stellar.org',\n    // Position defaults (matching your create-position form)\n    defaultMaturityDays: 30,\n    defaultCouponAmount: '1000',\n    defaultPrincipalAmount: '10000',\n    // Display\n    currency: 'USD',\n    numberFormat: 'standard',\n    showAdvancedOptions: false,\n    // Transactions\n    defaultFee: '100000',\n    defaultTimeout: 30,\n    confirmTransactions: true,\n    // Notifications\n    showSuccessNotifications: true,\n    showWarningNotifications: true,\n    showErrorNotifications: true,\n    // Developer\n    showContractAddresses: false,\n    enableDebugMode: false\n};\nconst SettingsContext = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_1__.createContext)(undefined);\nconst SettingsProvider = ({ children })=>{\n    const [settings, setSettings] = (0,react__WEBPACK_IMPORTED_MODULE_1__.useState)(defaultSettings);\n    // Load settings from localStorage on mount\n    (0,react__WEBPACK_IMPORTED_MODULE_1__.useEffect)({\n        \"SettingsProvider.useEffect\": ()=>{\n            try {\n                const savedSettings = localStorage.getItem('yieldback-settings');\n                if (savedSettings) {\n                    const parsed = JSON.parse(savedSettings);\n                    // Merge with defaults to handle new settings added in updates\n                    setSettings({\n                        \"SettingsProvider.useEffect\": (prev)=>({\n                                ...defaultSettings,\n                                ...parsed\n                            })\n                    }[\"SettingsProvider.useEffect\"]);\n                }\n            } catch (error) {\n                console.warn('Failed to load settings from localStorage:', error);\n            }\n        }\n    }[\"SettingsProvider.useEffect\"], []);\n    // Save settings to localStorage whenever they change\n    (0,react__WEBPACK_IMPORTED_MODULE_1__.useEffect)({\n        \"SettingsProvider.useEffect\": ()=>{\n            try {\n                localStorage.setItem('yieldback-settings', JSON.stringify(settings));\n            } catch (error) {\n                console.warn('Failed to save settings to localStorage:', error);\n            }\n        }\n    }[\"SettingsProvider.useEffect\"], [\n        settings\n    ]);\n    const updateSetting = (key, value)=>{\n        setSettings((prev)=>({\n                ...prev,\n                [key]: value\n            }));\n    };\n    const resetSettings = ()=>{\n        setSettings(defaultSettings);\n        try {\n            localStorage.removeItem('yieldback-settings');\n        } catch (error) {\n            console.warn('Failed to clear settings from localStorage:', error);\n        }\n    };\n    const exportSettings = ()=>{\n        return JSON.stringify(settings, null, 2);\n    };\n    const importSettings = (settingsJson)=>{\n        try {\n            const imported = JSON.parse(settingsJson);\n            // Validate that it's a valid settings object\n            if (typeof imported === 'object' && imported !== null) {\n                setSettings({\n                    ...defaultSettings,\n                    ...imported\n                });\n                return true;\n            }\n            return false;\n        } catch (error) {\n            console.error('Failed to import settings:', error);\n            return false;\n        }\n    };\n    return /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(SettingsContext.Provider, {\n        value: {\n            settings,\n            updateSetting,\n            resetSettings,\n            exportSettings,\n            importSettings\n        },\n        children: children\n    }, void 0, false, {\n        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\contexts\\\\settings.tsx\",\n        lineNumber: 141,\n        columnNumber: 5\n    }, undefined);\n};\nconst useSettings = ()=>{\n    const context = (0,react__WEBPACK_IMPORTED_MODULE_1__.useContext)(SettingsContext);\n    if (!context) {\n        throw new Error('useSettings must be used within a SettingsProvider');\n    }\n    return context;\n};\n// Specialized hooks for common use cases\nconst useTheme = ()=>{\n    const { settings, updateSetting } = useSettings();\n    return {\n        theme: settings.theme,\n        isDarkMode: settings.theme === 'dark',\n        toggleTheme: ()=>updateSetting('theme', settings.theme === 'light' ? 'dark' : 'light'),\n        setTheme: (theme)=>updateSetting('theme', theme)\n    };\n};\nconst useNetwork = ()=>{\n    const { settings, updateSetting } = useSettings();\n    return {\n        network: settings.network,\n        rpcUrl: settings.customRpcUrl,\n        isTestnet: settings.network === 'testnet',\n        setNetwork: (network)=>updateSetting('network', network),\n        setRpcUrl: (url)=>updateSetting('customRpcUrl', url),\n        getNetworkConfig: ()=>({\n                networkPassphrase: settings.network === 'testnet' ? \"Test SDF Network ; September 2015\" : \"Public Global Stellar Network ; September 2015\",\n                rpcUrl: settings.customRpcUrl\n            })\n    };\n};\nconst usePositionDefaults = ()=>{\n    const { settings, updateSetting } = useSettings();\n    return {\n        defaultMaturityDays: settings.defaultMaturityDays,\n        defaultCouponAmount: settings.defaultCouponAmount,\n        defaultPrincipalAmount: settings.defaultPrincipalAmount,\n        setDefaultMaturityDays: (days)=>updateSetting('defaultMaturityDays', days),\n        setDefaultCouponAmount: (amount)=>updateSetting('defaultCouponAmount', amount),\n        setDefaultPrincipalAmount: (amount)=>updateSetting('defaultPrincipalAmount', amount)\n    };\n};\nconst useTransactionSettings = ()=>{\n    const { settings, updateSetting } = useSettings();\n    return {\n        defaultFee: settings.defaultFee,\n        defaultTimeout: settings.defaultTimeout,\n        confirmTransactions: settings.confirmTransactions,\n        setDefaultFee: (fee)=>updateSetting('defaultFee', fee),\n        setDefaultTimeout: (timeout)=>updateSetting('defaultTimeout', timeout),\n        setConfirmTransactions: (confirm)=>updateSetting('confirmTransactions', confirm)\n    };\n};\nconst useNotifications = ()=>{\n    const { settings, updateSetting } = useSettings();\n    return {\n        showSuccess: settings.showSuccessNotifications,\n        showWarning: settings.showWarningNotifications,\n        showError: settings.showErrorNotifications,\n        setShowSuccess: (show)=>updateSetting('showSuccessNotifications', show),\n        setShowWarning: (show)=>updateSetting('showWarningNotifications', show),\n        setShowError: (show)=>updateSetting('showErrorNotifications', show)\n    };\n};\n//# sourceURL=[module]\n//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHBhZ2VzLWRpci1ub2RlKS8uL3NyYy9jb250ZXh0cy9zZXR0aW5ncy50c3giLCJtYXBwaW5ncyI6Ijs7Ozs7Ozs7Ozs7Ozs7O0FBQThFO0FBMkM5RSxNQUFNSyxrQkFBK0I7SUFDbkMsUUFBUTtJQUNSQyxPQUFPO0lBRVAsVUFBVTtJQUNWQyxTQUFTO0lBQ1RDLGNBQWM7SUFFZCx5REFBeUQ7SUFDekRDLHFCQUFxQjtJQUNyQkMscUJBQXFCO0lBQ3JCQyx3QkFBd0I7SUFFeEIsVUFBVTtJQUNWQyxVQUFVO0lBQ1ZDLGNBQWM7SUFDZEMscUJBQXFCO0lBRXJCLGVBQWU7SUFDZkMsWUFBWTtJQUNaQyxnQkFBZ0I7SUFDaEJDLHFCQUFxQjtJQUVyQixnQkFBZ0I7SUFDaEJDLDBCQUEwQjtJQUMxQkMsMEJBQTBCO0lBQzFCQyx3QkFBd0I7SUFFeEIsWUFBWTtJQUNaQyx1QkFBdUI7SUFDdkJDLGlCQUFpQjtBQUNuQjtBQUVBLE1BQU1DLGdDQUFrQnRCLG9EQUFhQSxDQUFrQ3VCO0FBRWhFLE1BQU1DLG1CQUE0RCxDQUFDLEVBQUVDLFFBQVEsRUFBRTtJQUNwRixNQUFNLENBQUNDLFVBQVVDLFlBQVksR0FBR3pCLCtDQUFRQSxDQUFjRTtJQUV0RCwyQ0FBMkM7SUFDM0NELGdEQUFTQTtzQ0FBQztZQUNSLElBQUk7Z0JBQ0YsTUFBTXlCLGdCQUFnQkMsYUFBYUMsT0FBTyxDQUFDO2dCQUMzQyxJQUFJRixlQUFlO29CQUNqQixNQUFNRyxTQUFTQyxLQUFLQyxLQUFLLENBQUNMO29CQUMxQiw4REFBOEQ7b0JBQzlERDtzREFBWU8sQ0FBQUEsT0FBUztnQ0FBRSxHQUFHOUIsZUFBZTtnQ0FBRSxHQUFHMkIsTUFBTTs0QkFBQzs7Z0JBQ3ZEO1lBQ0YsRUFBRSxPQUFPSSxPQUFPO2dCQUNkQyxRQUFRQyxJQUFJLENBQUMsOENBQThDRjtZQUM3RDtRQUNGO3FDQUFHLEVBQUU7SUFFTCxxREFBcUQ7SUFDckRoQyxnREFBU0E7c0NBQUM7WUFDUixJQUFJO2dCQUNGMEIsYUFBYVMs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LGNBQWMsMEJBQTBCNkI7SUFDekY7QUFDRixFQUFFO0FBRUssTUFBTUUseUJBQXlCO0lBQ3BDLE1BQU0sRUFBRTdDLFFBQVEsRUFBRWMsYUFBYSxFQUFFLEdBQUdVO0lBQ3BDLE9BQU87UUFDTHBDLFlBQVlZLFNBQVNaLFVBQVU7UUFDL0JDLGdCQUFnQlcsU0FBU1gsY0FBYztRQUN2Q0MscUJBQXFCVSxTQUFTVixtQkFBbUI7UUFDakR3RCxlQUFlLENBQUNDLE1BQWdCakMsY0FBYyxjQUFjaUM7UUFDNURDLG1CQUFtQixDQUFDQyxVQUFvQm5DLGNBQWMsa0JBQWtCbUM7UUFDeEVDLHdCQUF3QixDQUFDQyxVQUFxQnJDLGNBQWMsdUJBQXVCcUM7SUFDckY7QUFDRixFQUFFO0FBRUssTUFBTUMsbUJBQW1CO0lBQzlCLE1BQU0sRUFBRXBELFFBQVEsRUFBRWMsYUFBYSxFQUFFLEdBQUdVO0lBQ3BDLE9BQU87UUFDTDZCLGFBQWFyRCxTQUFTVCx3QkFBd0I7UUFDOUMrRCxhQUFhdEQsU0FBU1Isd0JBQXdCO1FBQzlDK0QsV0FBV3ZELFNBQVNQLHNCQUFzQjtRQUMxQytELGdCQUFnQixDQUFDQyxPQUFrQjNDLGNBQWMsNEJBQTRCMkM7UUFDN0VDLGdCQUFnQixDQUFDRCxPQUFrQjNDLGNBQWMsNEJBQTRCMkM7UUFDN0VFLGNBQWMsQ0FBQ0YsT0FBa0IzQyxjQUFjLDBCQUEwQjJDO0lBQzNFO0FBQ0YsRUFBRSIsInNvdXJjZXMiOlsiQzpcXGNvbXBzY2lcXGJsb2NrY2hhaW5cXHlpZWxkYmFja1xcZnJvbnRlbmRcXHNyY1xcY29udGV4dHNcXHNldHRpbmdzLnRzeCJdLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgUmVhY3QsIHsgY3JlYXRlQ29udGV4dCwgdXNlQ29udGV4dCwgdXNlU3RhdGUsIHVzZUVmZmVjdCB9IGZyb20gJ3JlYWN0JztcclxuXHJcbmludGVyZmFjZSBBcHBTZXR0aW5ncyB7XHJcbiAgLy8gVGhlbWUgcHJlZmVyZW5jZXNcclxuICB0aGVtZTogJ2xpZ2h0JyB8ICdkYXJrJztcclxuICBcclxuICAvLyBOZXR3b3JrIHByZWZlcmVuY2VzXHJcbiAgbmV0d29yazogJ3Rlc3RuZXQnIHwgJ21haW5uZXQnO1xyXG4gIGN1c3RvbVJwY1VybDogc3RyaW5nO1xyXG4gIFxyXG4gIC8vIFBvc2l0aW9uIGNyZWF0aW9uIGRlZmF1bHRzXHJcbiAgZGVmYXVsdE1hdHVyaXR5RGF5czogbnVtYmVyO1xyXG4gIGRlZmF1bHRDb3Vwb25BbW91bnQ6IHN0cmluZztcclxuICBkZWZhdWx0UHJpbmNpcGFsQW1vdW50OiBzdHJpbmc7XHJcbiAgXHJcbiAgLy8gRGlzcGxheSBwcmVmZXJlbmNlc1xyXG4gIGN1cnJlbmN5OiAnVVNEJyB8ICdFVVInIHwgJ1hMTSc7XHJcbiAgbnVtYmVyRm9ybWF0OiAnc3RhbmRhcmQnIHwgJ2NvbXBhY3QnO1xyXG4gIHNob3dBZHZhbmNlZE9wdGlvbnM6IGJvb2xlYW47XHJcbiAgXHJcbiAgLy8gVHJhbnNhY3Rpb24gcHJlZmVyZW5jZXNcclxuICBkZWZhdWx0RmVlOiBzdHJpbmc7XHJcbiAgZGVmYXVsdFRpbWVvdXQ6IG51bWJlcjtcclxuICBjb25maXJtVHJhbnNhY3Rpb25zOiBib29sZWFuO1xyXG4gIFxyXG4gIC8vIE5vdGlmaWNhdGlvbiBwcmVmZXJlbmNlc1xyXG4gIHNob3dTdWNjZXNzTm90aWZpY2F0aW9uczogYm9vbGVhbjtcclxuICBzaG93V2FybmluZ05vdGlmaWNhdGlvbnM6IGJvb2xlYW47XHJcbiAgc2hvd0Vycm9yTm90aWZpY2F0aW9uczogYm9vbGVhbjtcclxuICBcclxuICAvLyBEZXZlbG9wZXIgcHJlZmVyZW5jZXNcclxuICBzaG93Q29udHJhY3RBZGRyZXNzZXM6IGJvb2xlYW47XHJcbiAgZW5hYmxlRGVidWdNb2RlOiBib29sZWFuO1xyXG59XHJcblxyXG5pbnRlcmZhY2UgU2V0dGluZ3NDb250ZXh0VHlwZSB7XHJcbiAgc2V0dGluZ3M6IEFwcFNldHRpbmdzO1xyXG4gIHVwZGF0ZVNldHRpbmc6IDxLIGV4dGVuZHMga2V5b2YgQXBwU2V0dGluZ3M+KGtleTogSywgdmFsdWU6IEFwcFNldHRpbmdzW0tdKSA9PiB2b2lkO1xyXG4gIHJlc2V0U2V0dGluZ3M6ICgpID0+IHZvaWQ7XHJcbiAgZXhwb3J0U2V0dGluZ3M6ICgpID0+IHN0cmluZztcclxuICBpbXBvcnRTZXR0aW5nczogKHNldHRpbmdzSnNvbjogc3RyaW5nKSA9PiBib29sZWFuO1xyXG59XHJcblxyXG5jb25zdCBkZWZhdWx0U2V0dGluZ3M6IEFwcFNldHRpbmdzID0ge1xyXG4gIC8vIFRoZW1lXHJcbiAgdGhlbWU6ICdsaWdodCcsXHJcbiAgXHJcbiAgLy8gTmV0d29ya1xyXG4gIG5ldHdvcms6ICd0ZXN0bmV0JyxcclxuICBjdXN0b21ScGNVcmw6ICdodHRwczovL3Nvcm9iYW4tdGVzdG5ldC5zdGVsbGFyLm9yZycsXHJcbiAgXHJcbiAgLy8gUG9zaXRpb24gZGVmYXVsdHMgKG1hdGNoaW5nIHlvdXIgY3JlYXRlLXBvc2l0aW9uIGZvcm0pXHJcbiAgZGVmYXVsdE1hdHVyaXR5RGF5czogMzAsXHJcbiAgZGVmYXVsdENvdXBvbkFtb3VudDogJzEwMDAnLFxyXG4gIGRlZmF1bHRQcmluY2lwYWxBbW91bnQ6ICcxMDAwMCcsXHJcbiAgXHJcbiAgLy8gRGlzcGxheVxyXG4gIGN1cnJlbmN5OiAnVVNEJyxcclxuICBudW1iZXJGb3JtYXQ6ICdzdGFuZGFyZCcsXHJcbiAgc2hvd0FkdmFuY2VkT3B0aW9uczogZmFsc2UsXHJcbiAgXHJcbiAgLy8gVHJhbnNhY3Rpb25zXHJcbiAgZGVmYXVsdEZlZTogJzEwMDAwMCcsXHJcbiAgZGVmYXVsdFRpbWVvdXQ6IDMwLFxyXG4gIGNvbmZpcm1UcmFuc2FjdGlvbnM6IHRydWUsXHJcbiAgXHJcbiAgLy8gTm90aWZpY2F0aW9uc1xyXG4gIHNob3dTdWNjZXNzTm90aWZpY2F0aW9uczogdHJ1ZSxcclxuICBzaG93V2FybmluZ05vdGlmaWNhdGlvbnM6IHRydWUsXHJcbiAgc2hvd0Vycm9yTm90aWZpY2F0aW9uczogdHJ1ZSxcclxuICBcclxuICAvLyBEZXZlbG9wZXJcclxuICBzaG93Q29udHJhY3RBZGRyZXNzZXM6IGZhbHNlLFxyXG4gIGVuYWJsZURlYnVnTW9kZTogZmFsc2UsXHJcbn07XHJcblxyXG5jb25zdCBTZXR0aW5nc0NvbnRleHQgPSBjcmVhdGVDb250ZXh0PFNldHRpbmdzQ29udGV4dFR5cGUgfCB1bmRlZmluZWQ+KHVuZGVmaW5lZCk7XHJcblxyXG5leHBvcnQgY29uc3QgU2V0dGluZ3NQcm92aWRlcjogUmVhY3QuRkM8eyBjaGlsZHJlbjogUmVhY3QuUmVhY3ROb2RlIH0+ID0gKHsgY2hpbGRyZW4gfSkgPT4ge1xyXG4gIGNvbnN0IFtzZXR0aW5ncywgc2V0U2V0dGluZ3NdID0gdXNlU3RhdGU8QXBwU2V0dGluZ3M+KGRlZmF1bHRTZXR0aW5ncyk7XHJcblxyXG4gIC8vIExvYWQgc2V0dGluZ3MgZnJvbSBsb2NhbFN0b3JhZ2Ugb24gbW91bnRcclxuICB1c2VFZmZlY3QoKCkgPT4ge1xyXG4gICAgdHJ5IHtcclxuICAgICAgY29uc3Qgc2F2ZWRTZXR0aW5ncyA9IGxvY2FsU3RvcmFnZS5nZXRJdGVtKCd5aWVsZGJhY2stc2V0dGluZ3MnKTtcclxuICAgICAgaWYgKHNhdmVkU2V0dGluZ3MpIHtcclxuICAgICAgICBjb25zdCBwYXJzZWQgPSBKU09OLnBhcnNlKHNhdmVkU2V0dGluZ3MpO1xyXG4gICAgICAgIC8vIE1lcmdlIHdpdGggZGVmYXVsdHMgdG8gaGFuZGxlIG5ldyBzZXR0aW5ncyBhZGRlZCBpbiB1cGRhdGVzXHJcbiAgICAgICAgc2V0U2V0dGluZ3MocHJldiA9PiAoeyAuLi5kZWZhdWx0U2V0dGluZ3MsIC4uLnBhcnNlZCB9KSk7XHJcbiAgICAgIH1cclxuICAgIH0gY2F0Y2ggKGVycm9yKSB7XHJcbiAgICAgIGNvbnNvbGUud2FybignRmFpbGVkIHRvIGxvYWQgc2V0dGluZ3MgZnJvbSBsb2NhbFN0b3JhZ2U6JywgZXJyb3IpO1xyXG4gICAgfVxyXG4gIH0sIFtdKTtcclxuXHJcbiAgLy8gU2F2ZSBzZXR0aW5ncyB0byBsb2NhbFN0b3JhZ2Ugd2hlbmV2ZXIgdGhleSBjaGFuZ2VcclxuICB1c2VFZmZlY3QoKCkgPT4ge1xyXG4gICAgdHJ5IHtcclxuICAgICAgbG9jYWxTdG9yYWdlLnNldEl0ZW0oJ3lpZWxkYmFjay1zZXR0aW5ncycsIEpTT04uc3RyaW5naWZ5KHNldHRpbmdzKSk7XHJcbiAgICB9IGNhdGNoIChlcnJvcikge1xyXG4gICAgICBjb25zb2xlLndhcm4oJ0ZhaWxlZCB0byBzYXZlIHNldHRpbmdzIHRvIGxvY2FsU3RvcmFnZTonLCBlcnJvcik7XHJcbiAgICB9XHJcbiAgfSwgW3NldHRpbmdzXSk7XHJcblxyXG4gIGNvbnN0IHVwZGF0ZVNldHRpbmcgPSA8SyBleHRlbmRzIGtleW9mIEFwcFNldHRpbmdzPihrZXk6IEssIHZhbHVlOiBBcHBTZXR0aW5nc1tLXSkgPT4ge1xyXG4gICAgc2V0U2V0dGluZ3MocHJldiA9PiAoe1xyXG4gICAgICAuLi5wcmV2LFxyXG4gICAgICBba2V5XTogdmFsdWUsXHJcbiAgICB9KSk7XHJcbiAgfTtcclxuXHJcbiAgY29uc3QgcmVzZXRTZXR0aW5ncyA9ICgpID0+IHtcclxuICAgIHNldFNldHRpbmdzKGRlZmF1bHRTZXR0aW5ncyk7XHJcbiAgICB0cnkge1xyXG4gICAgICBsb2NhbFN0b3JhZ2UucmVtb3ZlSXRlbSgneWllbGRiYWNrLXNldHRpbmdzJyk7XHJcbiAgICB9IGNhdGNoIChlcnJvcikge1xyXG4gICAgICBjb25zb2xlLndhcm4oJ0ZhaWxlZCB0byBjbGVhciBzZXR0aW5ncyBmcm9tIGxvY2FsU3RvcmFnZTonLCBlcnJvcik7XHJcbiAgICB9XHJcbiAgfTtcclxuXHJcbiAgY29uc3QgZXhwb3J0U2V0dGluZ3MgPSAoKTogc3RyaW5nID0+IHtcclxuICAgIHJldHVybiBKU09OLnN0cmluZ2lmeShzZXR0aW5ncywgbnVsbCwgMik7XHJcbiAgfTtcclxuXHJcbiAgY29uc3QgaW1wb3J0U2V0dGluZ3MgPSAoc2V0dGluZ3NKc29uOiBzdHJpbmcpOiBib29sZWFuID0+IHtcclxuICAgIHRyeSB7XHJcbiAgICAgIGNvbnN0IGltcG9ydGVkID0gSlNPTi5wYXJzZShzZXR0aW5nc0pzb24pO1xyXG4gICAgICAvLyBWYWxpZGF0ZSB0aGF0IGl0J3MgYSB2YWxpZCBzZXR0aW5ncyBvYmplY3RcclxuICAgICAgaWYgKHR5cGVvZiBpbXBvcnRlZCA9PT0gJ29iamVjdCcgJiYgaW1wb3J0ZWQgIT09IG51bGwpIHtcclxuICAgICAgICBzZXRTZXR0aW5ncyh7IC4uLmRlZmF1bHRTZXR0aW5ncywgLi4uaW1wb3J0ZWQgfSk7XHJcbiAgICAgICAgcmV0dXJuIHRydWU7XHJcbiAgICAgIH1cclxuICAgICAgcmV0dXJuIGZhbHNlO1xyXG4gICAgfSBjYXRjaCAoZXJyb3IpIHtcclxuICAgICAgY29uc29sZS5lcnJvcignRmFpbGVkIHRvIGltcG9ydCBzZXR0aW5nczonLCBlcnJvcik7XHJcbiAgICAgIHJldHVybiBmYWxzZTtcclxuICAgIH1cclxuICB9O1xyXG5cclxuICByZXR1cm4gKFxyXG4gICAgPFNldHRpbmdzQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17eyBcclxuICAgICAgc2V0dGluZ3MsIFxyXG4gICAgICB1cGRhdGVTZXR0aW5nLCBcclxuICAgICAgcmVzZXRTZXR0aW5ncywgXHJcbiAgICAgIGV4cG9ydFNldHRpbmdzLCBcclxuICAgICAgaW1wb3J0U2V0dGluZ3MgXHJcbiAgICB9fT5cclxuICAgICAge2NoaWxkcmVufVxyXG4gICAgPC9TZXR0aW5nc0NvbnRleHQuUHJvdmlkZXI+XHJcbiAgKTtcclxufTtcclxuXHJcbmV4cG9ydCBjb25zdCB1c2VTZXR0aW5ncyA9ICgpOiBTZXR0aW5nc0NvbnRleHRUeXBlID0+IHtcclxuICBjb25zdCBjb250ZXh0ID0gdXNlQ29udGV4dChTZXR0aW5nc0NvbnRleHQpO1xyXG4gIGlmICghY29udGV4dCkge1xyXG4gICAgdGhyb3cgbmV3IEVycm9yKCd1c2VTZXR0aW5ncyBtdXN0IGJlIHVzZWQgd2l0aGluIGEgU2V0dGluZ3NQcm92aWRlcicpO1xyXG4gIH1cclxuICByZXR1cm4gY29udGV4dDtcclxufTtcclxuXHJcbi8vIFNwZWNpYWxpemVkIGhvb2tzIGZvciBjb21tb24gdXNlIGNhc2VzXHJcbmV4cG9ydCBjb25zdCB1c2VUaGVtZSA9ICgpID0+IHtcclxuICBjb25zdCB7IHNldHRpbmdzLCB1cGRhdGVTZXR0aW5nIH0gPSB1c2VTZXR0aW5ncygpO1xyXG4gIHJldHVybiB7XHJcbiAgICB0aGVtZTogc2V0dGluZ3MudGhlbWUsXHJcbiAgICBpc0RhcmtNb2RlOiBzZXR0aW5ncy50aGVtZSA9PT0gJ2RhcmsnLFxyXG4gICAgdG9nZ2xlVGhlbWU6ICgpID0+IHVwZGF0ZVNldHRpbmcoJ3RoZW1lJywgc2V0dGluZ3MudGhlbWUgPT09ICdsaWdodCcgPyAnZGFyaycgOiAnbGlnaHQnKSxcclxuICAgIHNldFRoZW1lOiAodGhlbWU6ICdsaWdodCcgfCAnZGFyaycpID0+IHVwZGF0ZVNldHRpbmcoJ3RoZW1lJywgdGhlbWUpLFxyXG4gIH07XHJcbn07XHJcblxyXG5leHBvcnQgY29uc3QgdXNlTmV0d29yayA9ICgpID0+IHtcclxuICBjb25zdCB7IHNldHRpbmdzLCB1cGRhdGVTZXR0aW5nIH0gPSB1c2VTZXR0aW5ncygpO1xyXG4gIHJldHVybiB7XHJcbiAgICBuZXR3b3JrOiBzZXR0aW5ncy5uZXR3b3JrLFxyXG4gICAgcnBjVXJsOiBzZXR0aW5ncy5jdXN0b21ScGNVcmwsXHJcbiAgICBpc1Rlc3RuZXQ6IHNldHRpbmdzLm5ldHdvcmsgPT09ICd0ZXN0bmV0JyxcclxuICAgIHNldE5ldHdvcms6IChuZXR3b3JrOiAndGVzdG5ldCcgfCAnbWFpbm5ldCcpID0+IHVwZGF0ZVNldHRpbmcoJ25ldHdvcmsnLCBuZXR3b3JrKSxcclxuICAgIHNldFJwY1VybDogKHVybDogc3RyaW5nKSA9PiB1cGRhdGVTZXR0aW5nKCdjdXN0b21ScGNVcmwnLCB1cmwpLFxyXG4gICAgZ2V0TmV0d29ya0NvbmZpZzogKCkgPT4gKHtcclxuICAgICAgbmV0d29ya1Bhc3NwaHJhc2U6IHNldHRpbmdzLm5ldHdvcmsgPT09ICd0ZXN0bmV0JyBcclxuICAgICAgICA/IFwiVGVzdCBTREYgTmV0d29yayA7IFNlcHRlbWJlciAyMDE1XCIgXHJcbiAgICAgICAgOiBcIlB1YmxpYyBHbG9iYWwgU3RlbGxhciBOZXR3b3JrIDsgU2VwdGVtYmVyIDIwMTVcIixcclxuICAgICAgcnBjVXJsOiBzZXR0aW5ncy5jdXN0b21ScGNVcmwsXHJcbiAgICB9KSxcclxuICB9O1xyXG59O1xyXG5cclxuZXhwb3J0IGNvbnN0IHVzZVBvc2l0aW9uRGVmYXVsdHMgPSAoKSA9PiB7XHJcbiAgY29uc3QgeyBzZXR0aW5ncywgdXBkYXRlU2V0dGluZyB9ID0gdXNlU2V0dGluZ3MoKTtcclxuICByZXR1cm4ge1xyXG4gICAgZGVmYXVsdE1hdHVyaXR5RGF5czogc2V0dGluZ3MuZGVmYXVsdE1hdHVyaXR5RGF5cyxcclxuICAgIGRlZmF1bHRDb3Vwb25BbW91bnQ6IHNldHRpbmdzLmRlZmF1bHRDb3Vwb25BbW91bnQsXHJcbiAgICBkZWZhdWx0UHJpbmNpcGFsQW1vdW50OiBzZXR0aW5ncy5kZWZhdWx0UHJpbmNpcGFsQW1vdW50LFxyXG4gICAgc2V0RGVmYXVsdE1hdHVyaXR5RGF5czogKGRheXM6IG51bWJlcikgPT4gdXBkYXRlU2V0dGluZygnZGVmYXVsdE1hdHVyaXR5RGF5cycsIGRheXMpLFxyXG4gICAgc2V0RGVmYXVsdENvdXBvbkFtb3VudDogKGFtb3VudDogc3RyaW5nKSA9PiB1cGRhdGVTZXR0aW5nKCdkZWZhdWx0Q291cG9uQW1vdW50JywgYW1vdW50KSxcclxuICAgIHNldERlZmF1bHRQcmluY2lwYWxBbW91bnQ6IChhbW91bnQ6IHN0cmluZykgPT4gdXBkYXRlU2V0dGluZygnZGVmYXVsdFByaW5jaXBhbEFtb3VudCcsIGFtb3VudCksXHJcbiAgfTtcclxufTtcclxuXHJcbmV4cG9ydCBjb25zdCB1c2VUcmFuc2FjdGlvblNldHRpbmdzID0gKCkgPT4ge1xyXG4gIGNvbnN0IHsgc2V0dGluZ3MsIHVwZGF0ZVNldHRpbmcgfSA9IHVzZVNldHRpbmdzKCk7XHJcbiAgcmV0dXJuIHtcclxuICAgIGRlZmF1bHRGZWU6IHNldHRpbmdzLmRlZmF1bHRGZWUsXHJcbiAgICBkZWZhdWx0VGltZW91dDogc2V0dGluZ3MuZGVmYXVsdFRpbWVvdXQsXHJcbiAgICBjb25maXJtVHJhbnNhY3Rpb25zOiBzZXR0aW5ncy5jb25maXJtVHJhbnNhY3Rpb25zLFxyXG4gICAgc2V0RGVmYXVsdEZlZTogKGZlZTogc3RyaW5nKSA9PiB1cGRhdGVTZXR0aW5nKCdkZWZhdWx0RmVlJywgZmVlKSxcclxuICAgIHNldERlZmF1bHRUaW1lb3V0OiAodGltZW91dDogbnVtYmVyKSA9PiB1cGRhdGVTZXR0aW5nKCdkZWZhdWx0VGltZW91dCcsIHRpbWVvdXQpLFxyXG4gICAgc2V0Q29uZmlybVRyYW5zYWN0aW9uczogKGNvbmZpcm06IGJvb2xlYW4pID0+IHVwZGF0ZVNldHRpbmcoJ2NvbmZpcm1UcmFuc2FjdGlvbnMnLCBjb25maXJtKSxcclxuICB9O1xyXG59O1xyXG5cclxuZXhwb3J0IGNvbnN0IHVzZU5vdGlmaWNhdGlvbnMgPSAoKSA9PiB7XHJcbiAgY29uc3QgeyBzZXR0aW5ncywgdXBkYXRlU2V0dGluZyB9ID0gdXNlU2V0dGluZ3MoKTtcclxuICByZXR1cm4ge1xyXG4gICAgc2hvd1N1Y2Nlc3M6IHNldHRpbmdzLnNob3dTdWNjZXNzTm90aWZpY2F0aW9ucyxcclxuICAgIHNob3dXYXJuaW5nOiBzZXR0aW5ncy5zaG93V2FybmluZ05vdGlmaWNhdGlvbnMsXHJcbiAgICBzaG93RXJyb3I6IHNldHRpbmdzLnNob3dFcnJvck5vdGlmaWNhdGlvbnMsXHJcbiAgICBzZXRTaG93U3VjY2VzczogKHNob3c6IGJvb2xlYW4pID0+IHVwZGF0ZVNldHRpbmcoJ3Nob3dTdWNjZXNzTm90aWZpY2F0aW9ucycsIHNob3cpLFxyXG4gICAgc2V0U2hvd1dhcm5pbmc6IChzaG93OiBib29sZWFuKSA9PiB1cGRhdGVTZXR0aW5nKCdzaG93V2FybmluZ05vdGlmaWNhdGlvbnMnLCBzaG93KSxcclxuICAgIHNldFNob3dFcnJvcjogKHNob3c6IGJvb2xlYW4pID0+IHVwZGF0ZVNldHRpbmcoJ3Nob3dFcnJvck5vdGlmaWNhdGlvbnMnLCBzaG93KSxcclxuICB9O1xyXG59OyJdLCJuYW1lcyI6WyJSZWFjdCIsImNyZWF0ZUNvbnRleHQiLCJ1c2VDb250ZXh0IiwidXNlU3RhdGUiLCJ1c2VFZmZlY3QiLCJkZWZhdWx0U2V0dGluZ3MiLCJ0aGVtZSIsIm5ldHdvcmsiLCJjdXN0b21ScGNVcmwiLCJkZWZhdWx0TWF0dXJpdHlEYXlzIiwiZGVmYXVsdENvdXBvbkFtb3VudCIsImRlZmF1bHRQcmluY2lwYWxBbW91bnQiLCJjdXJyZW5jeSIsIm51bWJlckZvcm1hdCIsInNob3dBZHZhbmNlZE9wdGlvbnMiLCJkZWZhdWx0RmVlIiwiZGVmYXVsdFRpbWVvdXQiLCJjb25maXJtVHJhbnNhY3Rpb25zIiwic2hvd1N1Y2Nlc3NOb3RpZmljYXRpb25zIiwic2hvd1dhcm5pbmdOb3RpZmljYXRpb25zIiwic2hvd0Vycm9yTm90aWZpY2F0aW9ucyIsInNob3dDb250cmFjdEFkZHJlc3NlcyIsImVuYWJsZURlYnVnTW9kZSIsIlNldHRpbmdzQ29udGV4dCIsInVuZGVmaW5lZCIsIlNldHRpbmdzUHJvdmlkZXIiLCJjaGlsZHJlbiIsInNldHRpbmdzIiwic2V0U2V0dGluZ3MiLCJzYXZlZFNldHRpbmdzIiwibG9jYWxTdG9yYWdlIiwiZ2V0SXRlbSIsInBhcnNlZCIsIkpTT04iLCJwYXJzZSIsInByZXYiLCJlcnJvciIsImNvbnNvbGUiLCJ3YXJuIiwic2V0SXRlbSIsInN0cmluZ2lmeSIsInVwZGF0ZVNldHRpbmciLCJrZXkiLCJ2YWx1ZSIsInJlc2V0U2V0dGluZ3MiLCJyZW1vdmVJdGVtIiwiZXhwb3J0U2V0dGluZ3MiLCJpbXBvcnRTZXR0aW5ncyIsInNldHRpbmdzSnNvbiIsImltcG9ydGVkIiwiUHJvdmlkZXIiLCJ1c2VTZXR0aW5ncyIsImNvbnRleHQiLCJFcnJvciIsInVzZVRoZW1lIiwiaXNEYXJrTW9kZSIsInRvZ2dsZVRoZW1lIiwic2V0VGhlbWUiLCJ1c2VOZXR3b3JrIiwicnBjVXJsIiwiaXNUZXN0bmV0Iiwic2V0TmV0d29yayIsInNldFJwY1VybCIsInVybCIsImdldE5ldHdvcmtDb25maWciLCJuZXR3b3JrUGFzc3BocmFzZSIsInVzZVBvc2l0aW9uRGVmYXVsdHMiLCJzZXREZWZhdWx0TWF0dXJpdHlEYXlzIiwiZGF5cyIsInNldERlZmF1bHRDb3Vwb25BbW91bnQiLCJhbW91bnQiLCJzZXREZWZhdWx0UHJpbmNpcGFsQW1vdW50IiwidXNlVHJhbnNhY3Rpb25TZXR0aW5ncyIsInNldERlZmF1bHRGZWUiLCJmZWUiLCJzZXREZWZhdWx0VGltZW91dCIsInRpbWVvdXQiLCJzZXRDb25maXJtVHJhbnNhY3Rpb25zIiwiY29uZmlybSIsInVzZU5vdGlmaWNhdGlvbnMiLCJzaG93U3VjY2VzcyIsInNob3dXYXJuaW5nIiwic2hvd0Vycm9yIiwic2V0U2hvd1N1Y2Nlc3MiLCJzaG93Iiwic2V0U2hvd1dhcm5pbmciLCJzZXRTaG93RXJyb3IiXSwiaWdub3JlTGlzdCI6W10sInNvdXJjZVJvb3QiOiIifQ==\n//# sourceURL=webpack-internal:///(pages-dir-node)/./src/contexts/settings.tsx\n");

/***/ }),

/***/ "(pages-dir-node)/./src/contexts/wallet.tsx":
/*!*********************************!*\
  !*** ./src/contexts/wallet.tsx ***!
  \*********************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   WalletProvider: () => (/* binding */ WalletProvider),\n/* harmony export */   useWallet: () => (/* binding */ useWallet),\n/* harmony export */   useWalletOperations: () => (/* binding */ useWalletOperations)\n/* harmony export */ });\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! react/jsx-dev-runtime */ \"react/jsx-dev-runtime\");\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! react */ \"react\");\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_1__);\n/* harmony import */ var _creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @creit.tech/stellar-wallets-kit */ \"@creit.tech/stellar-wallets-kit\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__]);\n_creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n\n// Initialize the wallet kit with hardcoded testnet configuration\nconst walletKit = new _creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__.StellarWalletsKit({\n    network: _creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__.WalletNetwork.TESTNET,\n    selectedWalletId: _creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__.XBULL_ID,\n    modules: [\n        new _creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__.xBullModule(),\n        new _creit_tech_stellar_wallets_kit__WEBPACK_IMPORTED_MODULE_2__.FreighterModule()\n    ]\n});\nconst WalletContext = /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_1___default().createContext(undefined);\nconst WalletProvider = ({ children = null })=>{\n    const [connected, setConnected] = (0,react__WEBPACK_IMPORTED_MODULE_1__.useState)(false);\n    const [loading, setLoading] = (0,react__WEBPACK_IMPORTED_MODULE_1__.useState)(false);\n    const [walletAddress, setWalletAddress] = (0,react__WEBPACK_IMPORTED_MODULE_1__.useState)('');\n    // Check if wallet was previously connected on mount\n    (0,react__WEBPACK_IMPORTED_MODULE_1__.useEffect)({\n        \"WalletProvider.useEffect\": ()=>{\n            checkConnection();\n        }\n    }[\"WalletProvider.useEffect\"], []);\n    const checkConnection = async ()=>{\n        try {\n            // Check if there's a stored wallet connection\n            const storedAddress = localStorage.getItem('walletAddress');\n            const storedWalletId = localStorage.getItem('selectedWalletId');\n            if (storedAddress && storedWalletId) {\n                // Set the wallet and try to verify the connection\n                try {\n                    walletKit.setWallet(storedWalletId);\n                    const { address } = await walletKit.getAddress();\n                    if (address) {\n                        setWalletAddress(address);\n                        setConnected(true);\n                    } else {\n                        // Clear stored address if connection is invalid\n                        localStorage.removeItem('walletAddress');\n                        localStorage.removeItem('selectedWalletId');\n                    }\n                } catch (error) {\n                    // Connection is no longer valid\n                    localStorage.removeItem('walletAddress');\n                    localStorage.removeItem('selectedWalletId');\n                    console.log('Previous connection no longer valid:', error);\n                }\n            }\n        } catch (error) {\n            console.log('No previous wallet connection found');\n        }\n    };\n    const connect = async ()=>{\n        try {\n            setLoading(true);\n            await walletKit.openModal({\n                onWalletSelected: async (option)=>{\n                    try {\n                        walletKit.setWallet(option.id);\n                        const { address } = await walletKit.getAddress();\n                        if (address) {\n                            setWalletAddress(address);\n                            setConnected(true);\n                            // Store the connection for persistence\n                            localStorage.setItem('walletAddress', address);\n                            localStorage.setItem('selectedWalletId', option.id);\n                        }\n                    } catch (error) {\n                        console.error('Error getting wallet address:', error);\n                        throw error;\n                    }\n                },\n                onClosed: (err)=>{\n                    setLoading(false);\n                    if (err) {\n                        console.error('Modal closed with error:', err);\n                    }\n                }\n            });\n        } catch (error) {\n            console.error('Unable to connect wallet:', error);\n            setLoading(false);\n            throw error;\n        }\n    };\n    const disconnect = ()=>{\n        setWalletAddress('');\n        setConnected(false);\n        // Clear stored connection data\n        localStorage.removeItem('walletAddress');\n        localStorage.removeItem('selectedWalletId');\n        // Reset wallet kit\n        try {\n            walletKit.setWallet('');\n        } catch (error) {\n            console.log('Error resetting wallet kit:', error);\n        }\n    };\n    // Handle wallet events (like account changes)\n    (0,react__WEBPACK_IMPORTED_MODULE_1__.useEffect)({\n        \"WalletProvider.useEffect\": ()=>{\n            const handleAccountChange = {\n                \"WalletProvider.useEffect.handleAccountChange\": ()=>{\n                    // If wallet changes accounts, update our state\n                    checkConnection();\n                }\n            }[\"WalletProvider.useEffect.handleAccountChange\"];\n            // Listen for wallet events if available\n            if (false) {}\n        }\n    }[\"WalletProvider.useEffect\"], []);\n    const contextValue = {\n        connected,\n        walletAddress,\n        isLoading: loading,\n        connect,\n        disconnect,\n        walletKit\n    };\n    return /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(WalletContext.Provider, {\n        value: contextValue,\n        children: children\n    }, void 0, false, {\n        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\contexts\\\\wallet.tsx\",\n        lineNumber: 149,\n        columnNumber: 5\n    }, undefined);\n};\nconst useWallet = ()=>{\n    const context = (0,react__WEBPACK_IMPORTED_MODULE_1__.useContext)(WalletContext);\n    if (!context) {\n        throw new Error('useWallet must be used within a WalletProvider');\n    }\n    return context;\n};\n// Helper hook for wallet operations\nconst useWalletOperations = ()=>{\n    const { walletKit, connected, walletAddress } = useWallet();\n    const signTransaction = async (xdr)=>{\n        if (!walletKit || !connected) {\n            throw new Error('Wallet not connected');\n        }\n        try {\n            const result = await walletKit.signTransaction(xdr, {\n                address: walletAddress,\n                networkPassphrase: \"Test SDF Network ; September 2015\"\n            });\n            return result.signedTxXdr;\n        } catch (error) {\n            console.error('Error signing transaction:', error);\n            throw error;\n        }\n    };\n    const getPublicKey = ()=>{\n        if (!connected || !walletAddress) {\n            throw new Error('Wallet not connected');\n        }\n        return walletAddress;\n    };\n    return {\n        signTransaction,\n        getPublicKey,\n        isReady: connected && walletKit !== null\n    };\n};\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });//# sourceURL=[module]\n//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHBhZ2VzLWRpci1ub2RlKS8uL3NyYy9jb250ZXh0cy93YWxsZXQudHN4IiwibWFwcGluZ3MiOiI7Ozs7Ozs7Ozs7Ozs7OztBQUErRDtBQVF0QjtBQVd6QyxpRUFBaUU7QUFDakUsTUFBTVMsWUFBK0IsSUFBSUwsOEVBQWlCQSxDQUFDO0lBQ3pETSxTQUFTTCwwRUFBYUEsQ0FBQ00sT0FBTztJQUM5QkMsa0JBQWtCTCxxRUFBUUE7SUFDMUJNLFNBQVM7UUFDUCxJQUFJTCx3RUFBV0E7UUFDZixJQUFJRiw0RUFBZUE7S0FDcEI7QUFDSDtBQUVBLE1BQU1RLDhCQUFnQmQsMERBQW1CLENBQTZCZ0I7QUFFL0QsTUFBTUMsaUJBQWlCLENBQUMsRUFBRUMsV0FBVyxJQUFXLEVBQUU7SUFDdkQsTUFBTSxDQUFDQyxXQUFXQyxhQUFhLEdBQUdsQiwrQ0FBUUEsQ0FBVTtJQUNwRCxNQUFNLENBQUNtQixTQUFTQyxXQUFXLEdBQUdwQiwrQ0FBUUEsQ0FBVTtJQUNoRCxNQUFNLENBQUNxQixlQUFlQyxpQkFBaUIsR0FBR3RCLCtDQUFRQSxDQUFTO0lBRTNELG9EQUFvRDtJQUNwREMsZ0RBQVNBO29DQUFDO1lBQ1JzQjtRQUNGO21DQUFHLEVBQUU7SUFFTCxNQUFNQSxrQkFBa0I7UUFDdEIsSUFBSTtZQUNGLDhDQUE4QztZQUM5QyxNQUFNQyxnQkFBZ0JDLGFBQWFDLE9BQU8sQ0FBQztZQUMzQyxNQUFNQyxpQkFBaUJGLGFBQWFDLE9BQU8sQ0FBQztZQUU1QyxJQUFJRixpQkFBaUJHLGdCQUFnQjtnQkFDbkMsa0RBQWtEO2dCQUNsRCxJQUFJO29CQUNGcEIsVUFBVXFCLFNBQVMsQ0FBQ0Q7b0JBQ3BCLE1BQU0sRUFBRUUsT0FBTyxFQUFFLEdBQUcsTUFBTXRCLFVBQVV1QixVQUFVO29CQUM5QyxJQUFJRCxTQUFTO3dCQUNYUCxpQkFBaUJPO3dCQUNqQlgsYUFBYTtvQkFDZixPQUFPO3dCQUNMLGdEQUFnRDt3QkFDaERPLGFBQWFNLFVBQVUsQ0FBQzt3QkFDeEJOLGFBQWFNLFVB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LGVBQWU7WUFDaEMsTUFBTSxJQUFJaUMsTUFBTTtRQUNsQjtRQUNBLE9BQU9qQztJQUNUO0lBRUEsT0FBTztRQUNMbUM7UUFDQUs7UUFDQUMsU0FBUzdDLGFBQWFWLGNBQWM7SUFDdEM7QUFDRixFQUFFIiwic291cmNlcyI6WyJDOlxcY29tcHNjaVxcYmxvY2tjaGFpblxceWllbGRiYWNrXFxmcm9udGVuZFxcc3JjXFxjb250ZXh0c1xcd2FsbGV0LnRzeCJdLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgUmVhY3QsIHsgdXNlQ29udGV4dCwgdXNlU3RhdGUsIHVzZUVmZmVjdCB9IGZyb20gJ3JlYWN0JztcclxuaW1wb3J0IHtcclxuICBTdGVsbGFyV2FsbGV0c0tpdCxcclxuICBXYWxsZXROZXR3b3JrLFxyXG4gIEZyZWlnaHRlck1vZHVsZSxcclxuICBYQlVMTF9JRCxcclxuICB4QnVsbE1vZHVsZSxcclxuICBJU3VwcG9ydGVkV2FsbGV0LFxyXG59IGZyb20gJ0BjcmVpdC50ZWNoL3N0ZWxsYXItd2FsbGV0cy1raXQnO1xyXG5cclxuZXhwb3J0IGludGVyZmFjZSBJV2FsbGV0Q29udGV4dCB7XHJcbiAgY29ubmVjdGVkOiBib29sZWFuO1xyXG4gIHdhbGxldEFkZHJlc3M6IHN0cmluZztcclxuICBpc0xvYWRpbmc6IGJvb2xlYW47XHJcbiAgY29ubmVjdDogKCkgPT4gUHJvbWlzZTx2b2lkPjtcclxuICBkaXNjb25uZWN0OiAoKSA9PiB2b2lkO1xyXG4gIHdhbGxldEtpdDogU3RlbGxhcldhbGxldHNLaXQgfCBudWxsO1xyXG59XHJcblxyXG4vLyBJbml0aWFsaXplIHRoZSB3YWxsZXQga2l0IHdpdGggaGFyZGNvZGVkIHRlc3RuZXQgY29uZmlndXJhdGlvblxyXG5jb25zdCB3YWxsZXRLaXQ6IFN0ZWxsYXJXYWxsZXRzS2l0ID0gbmV3IFN0ZWxsYXJXYWxsZXRzS2l0KHtcclxuICBuZXR3b3JrOiBXYWxsZXROZXR3b3JrLlRFU1RORVQsXHJcbiAgc2VsZWN0ZWRXYWxsZXRJZDogWEJVTExfSUQsXHJcbiAgbW9kdWxlczogW1xyXG4gICAgbmV3IHhCdWxsTW9kdWxlKCksXHJcbiAgICBuZXcgRnJlaWdodGVyTW9kdWxlKCksXHJcbiAgXSxcclxufSk7XHJcblxyXG5jb25zdCBXYWxsZXRDb250ZXh0ID0gUmVhY3QuY3JlYXRlQ29udGV4dDxJV2FsbGV0Q29udGV4dCB8IHVuZGVmaW5lZD4odW5kZWZpbmVkKTtcclxuXHJcbmV4cG9ydCBjb25zdCBXYWxsZXRQcm92aWRlciA9ICh7IGNoaWxkcmVuID0gbnVsbCBhcyBhbnkgfSkgPT4ge1xyXG4gIGNvbnN0IFtjb25uZWN0ZWQsIHNldENvbm5lY3RlZF0gPSB1c2VTdGF0ZTxib29sZWFuPihmYWxzZSk7XHJcbiAgY29uc3QgW2xvYWRpbmcsIHNldExvYWRpbmddID0gdXNlU3RhdGU8Ym9vbGVhbj4oZmFsc2UpO1xyXG4gIGNvbnN0IFt3YWxsZXRBZGRyZXNzLCBzZXRXYWxsZXRBZGRyZXNzXSA9IHVzZVN0YXRlPHN0cmluZz4oJycpO1xyXG5cclxuICAvLyBDaGVjayBpZiB3YWxsZXQgd2FzIHByZXZpb3VzbHkgY29ubmVjdGVkIG9uIG1vdW50XHJcbiAgdXNlRWZmZWN0KCgpID0+IHtcclxuICAgIGNoZWNrQ29ubmVjdGlvbigpO1xyXG4gIH0sIFtdKTtcclxuXHJcbiAgY29uc3QgY2hlY2tDb25uZWN0aW9uID0gYXN5bmMgKCkgPT4ge1xyXG4gICAgdHJ5IHtcclxuICAgICAgLy8gQ2hlY2sgaWYgdGhlcmUncyBhIHN0b3JlZCB3YWxsZXQgY29ubmVjdGlvblxyXG4gICAgICBjb25zdCBzdG9yZWRBZGRyZXNzID0gbG9jYWxTdG9yYWdlLmdldEl0ZW0oJ3dhbGxldEFkZHJlc3MnKTtcclxuICAgICAgY29uc3Qgc3RvcmVkV2FsbGV0SWQgPSBsb2NhbFN0b3JhZ2UuZ2V0SXRlbSgnc2VsZWN0ZWRXYWxsZXRJZCcpO1xyXG4gICAgICBcclxuICAgICAgaWYgKHN0b3JlZEFkZHJlc3MgJiYgc3RvcmVkV2FsbGV0SWQpIHtcclxuICAgICAgICAvLyBTZXQgdGhlIHdhbGxldCBhbmQgdHJ5IHRvIHZlcmlmeSB0aGUgY29ubmVjdGlvblxyXG4gICAgICAgIHRyeSB7XHJcbiAgICAgICAgICB3YWxsZXRLaXQuc2V0V2FsbGV0KHN0b3JlZFdhbGxldElkKTtcclxuICAgICAgICAgIGNvbnN0IHsgYWRkcmVzcyB9ID0gYXdhaXQgd2FsbGV0S2l0LmdldEFkZHJlc3MoKTtcclxuICAgICAgICAgIGlmIChhZGRyZXNzKSB7XHJcbiAgICAgICAgICAgIHNldFdhbGxldEFkZHJlc3MoYWRkcmVzcyk7XHJcbiAgICAgICAgICAgIHNldENvbm5lY3RlZCh0cnVlKTtcclxuICAgICAgICAgIH0gZWxzZSB7XHJcbiAgICAgICAgICAgIC8vIENsZWFyIHN0b3JlZCBhZGRyZXNzIGlmIGNvbm5lY3Rpb24gaXMgaW52YWxpZFxyXG4gICAgICAgICAgICBsb2NhbFN0b3JhZ2UucmVtb3ZlSXRlbSgnd2FsbGV0QWRkcmVzcycpO1xyXG4gICAgICAgICAgICBsb2NhbFN0b3JhZ2UucmVtb3ZlSXRlbSgnc2VsZWN0ZWRXYWxsZXRJZCcpO1xyXG4gICAgICAgICAgfVxyXG4gICAgICAgIH0gY2F0Y2ggKGVycm9yKSB7XHJcbiAgICAgICAgICAvLyBDb25uZWN0aW9uIGlzIG5vIGxvbmdlciB2YWxpZFxyXG4gICAgICAgICAgbG9jYWxTdG9yYWdlLnJlbW92ZUl0ZW0oJ3dhbGxldEFkZHJlc3MnKTtcclxuICAgICAgICAgIGxvY2FsU3RvcmFnZS5yZW1vdmVJdGVtKCdzZWxlY3RlZFdhbGxldElkJyk7XHJcbiAgICAgICAgICBjb25zb2xlLmxvZygnUHJldmlvdXMgY29ubmVjdGlvbiBubyBsb25nZXIgdmFsaWQ6JywgZXJyb3IpO1xyXG4gICAgICAgIH1cclxuICAgICAgfVxyXG4gICAgfSBjYXRjaCAoZXJyb3IpIHtcclxuICAgICAgY29uc29sZS5sb2coJ05vIHByZXZpb3VzIHdhbGxldCBjb25uZWN0aW9uIGZvdW5kJyk7XHJcbiAgICB9XHJcbiAgfTtcclxuXHJcbiAgY29uc3QgY29ubmVjdCA9IGFzeW5jICgpOiBQcm9taXNlPHZvaWQ+ID0+IHtcclxuICAgIHRyeSB7XHJcbiAgICAgIHNldExvYWRpbmcodHJ1ZSk7XHJcbiAgICAgIGF3YWl0IHdhbGxldEtpdC5vcGVuTW9kYWwoe1xyXG4gICAgICAgIG9uV2FsbGV0U2VsZWN0ZWQ6IGFzeW5jIChvcHRpb246IElTdXBwb3J0ZWRXYWxsZXQpID0+IHtcclxuICAgICAgICAgIHRyeSB7XHJcbiAgICAgICAgICAgIHdhbGxldEtpdC5zZXRXYWxsZXQob3B0aW9uLmlkKTtcclxuICAgICAgICAgICAgY29uc3QgeyBhZGRyZXNzIH0gPSBhd2FpdCB3YWxsZXRLaXQuZ2V0QWRkcmVzcygpO1xyXG4gICAgICAgICAgICBpZiAoYWRkcmVzcykge1xyXG4gICAgICAgICAgICAgIHNldFdhbGxldEFkZHJlc3MoYWRkcmVzcyk7XHJcbiAgICAgICAgICAgICAgc2V0Q29ubmVjdGVkKHRydWUpO1xyXG4gICAgICAgICAgICAgIC8vIFN0b3JlIHRoZSBjb25uZWN0aW9uIGZvciBwZXJzaXN0ZW5jZVxyXG4gICAgICAgICAgICAgIGxvY2FsU3RvcmFnZS5zZXRJdGVtKCd3YWxsZXRBZGRyZXNzJywgYWRkcmVzcyk7XHJcbiAgICAgICAgICAgICAgbG9jYWxTdG9yYWdlLnNldEl0ZW0oJ3NlbGVjdGVkV2FsbGV0SWQnLCBvcHRpb24uaWQpO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgICB9IGNhdGNoIChlcnJvcikge1xyXG4gICAgICAgICAgICBjb25zb2xlLmVycm9yKCdFcnJvciBnZXR0aW5nIHdhbGxldCBhZGRyZXNzOicsIGVycm9yKTtcclxuICAgICAgICAgICAgdGhyb3cgZXJyb3I7XHJcbiAgICAgICAgICB9XHJcbiAgICAgICAgfSxcclxuICAgICAgICBvbkNsb3NlZDogKGVycj86IEVycm9yKSA9PiB7XHJcbiAgICAgICAgICBzZXRMb2FkaW5nKGZhbHNlKTtcclxuICAgICAgICAgIGlmIChlcnIpIHtcclxuICAgICAgICAgICAgY29uc29sZS5lcnJvcignTW9kYWwgY2xvc2VkIHdpdGggZXJyb3I6JywgZXJyKTtcclxuICAgICAgICAgIH1cclxuICAgICAgICB9XHJcbiAgICAgIH0pO1xyXG4gICAgfSBjYXRjaCAoZXJyb3IpIHtcclxuICAgICAgY29uc29sZS5lcnJvcignVW5hYmxlIHRvIGNvbm5lY3Qgd2FsbGV0OicsIGVycm9yKTtcclxuICAgICAgc2V0TG9hZGluZyhmYWxzZSk7XHJcbiAgICAgIHRocm93IGVycm9yO1xyXG4gICAgfVxyXG4gIH07XHJcblxyXG4gIGNvbnN0IGRpc2Nvbm5lY3QgPSAoKTogdm9pZCA9PiB7XHJcbiAgICBzZXRXYWxsZXRBZGRyZXNzKCcnKTtcclxuICAgIHNldENvbm5lY3RlZChmYWxzZSk7XHJcbiAgICAvLyBDbGVhciBzdG9yZWQgY29ubmVjdGlvbiBkYXRhXHJcbiAgICBsb2NhbFN0b3JhZ2UucmVtb3ZlSXRlbSgnd2FsbGV0QWRkcmVzcycpO1xyXG4gICAgbG9jYWxTdG9yYWdlLnJlbW92ZUl0ZW0oJ3NlbGVjdGVkV2FsbGV0SWQnKTtcclxuICAgIFxyXG4gICAgLy8gUmVzZXQgd2FsbGV0IGtpdFxyXG4gICAgdHJ5IHtcclxuICAgICAgd2FsbGV0S2l0LnNldFdhbGxldCgnJyk7XHJcbiAgICB9IGNhdGNoIChlcnJvcikge1xyXG4gICAgICBjb25zb2xlLmxvZygnRXJyb3IgcmVzZXR0aW5nIHdhbGxldCBraXQ6JywgZXJyb3IpO1xyXG4gICAgfVxyXG4gIH07XHJcblxyXG4gIC8vIEhhbmRsZSB3YWxsZXQgZXZlbnRzIChsaWtlIGFjY291bnQgY2hhbmdlcylcclxuICB1c2VFZmZlY3QoKCkgPT4ge1xyXG4gICAgY29uc3QgaGFuZGxlQWNjb3VudENoYW5nZSA9ICgpID0+IHtcclxuICAgICAgLy8gSWYgd2FsbGV0IGNoYW5nZXMgYWNjb3VudHMsIHVwZGF0ZSBvdXIgc3RhdGVcclxuICAgICAgY2hlY2tDb25uZWN0aW9uKCk7XHJcbiAgICB9O1xyXG5cclxuICAgIC8vIExpc3RlbiBmb3Igd2FsbGV0IGV2ZW50cyBpZiBhdmFpbGFibGVcclxuICAgIGlmICh0eXBlb2Ygd2luZG93ICE9PSAndW5kZWZpbmVkJyAmJiB3aW5kb3cuYWRkRXZlbnRMaXN0ZW5lcikge1xyXG4gICAgICB3aW5kb3cuYWRkRXZlbnRMaXN0ZW5lcignc3RlbGxhcl93YWxsZXRfY2hhbmdlZCcsIGhhbmRsZUFjY291bnRDaGFuZ2UpO1xyXG4gICAgICBcclxuICAgICAgcmV0dXJuICgpID0+IHtcclxuICAgICAgICB3aW5kb3cucmVtb3ZlRXZlbnRMaXN0ZW5lcignc3RlbGxhcl93YWxsZXRfY2hhbmdlZCcsIGhhbmRsZUFjY291bnRDaGFuZ2UpO1xyXG4gICAgICB9O1xyXG4gICAgfVxyXG4gIH0sIFtdKTtcclxuXHJcbiAgY29uc3QgY29udGV4dFZhbHVlOiBJV2FsbGV0Q29udGV4dCA9IHtcclxuICAgIGNvbm5lY3RlZCxcclxuICAgIHdhbGxldEFkZHJlc3MsXHJcbiAgICBpc0xvYWRpbmc6IGxvYWRpbmcsXHJcbiAgICBjb25uZWN0LFxyXG4gICAgZGlzY29ubmVjdCxcclxuICAgIHdhbGxldEtpdCxcclxuICB9O1xyXG5cclxuICByZXR1cm4gKFxyXG4gICAgPFdhbGxldENvbnRleHQuUHJvdmlkZXIgdmFsdWU9e2NvbnRleHRWYWx1ZX0+XHJcbiAgICAgIHtjaGlsZHJlbn1cclxuICAgIDwvV2FsbGV0Q29udGV4dC5Qcm92aWRlcj5cclxuICApO1xyXG59O1xyXG5cclxuZXhwb3J0IGNvbnN0IHVzZVdhbGxldCA9ICgpOiBJV2FsbGV0Q29udGV4dCA9PiB7XHJcbiAgY29uc3QgY29udGV4dCA9IHVzZUNvbnRleHQoV2FsbGV0Q29udGV4dCk7XHJcbiAgaWYgKCFjb250ZXh0KSB7XHJcbiAgICB0aHJvdyBuZXcgRXJyb3IoJ3VzZVdhbGxldCBtdXN0IGJlIHVzZWQgd2l0aGluIGEgV2FsbGV0UHJvdmlkZXInKTtcclxuICB9XHJcbiAgcmV0dXJuIGNvbnRleHQ7XHJcbn07XHJcblxyXG4vLyBIZWxwZXIgaG9vayBmb3Igd2FsbGV0IG9wZXJhdGlvbnNcclxuZXhwb3J0IGNvbnN0IHVzZVdhbGxldE9wZXJhdGlvbnMgPSAoKSA9PiB7XHJcbiAgY29uc3QgeyB3YWxsZXRLaXQsIGNvbm5lY3RlZCwgd2FsbGV0QWRkcmVzcyB9ID0gdXNlV2FsbGV0KCk7XHJcblxyXG4gIGNvbnN0IHNpZ25UcmFuc2FjdGlvbiA9IGFzeW5jICh4ZHI6IHN0cmluZykgPT4ge1xyXG4gICAgaWYgKCF3YWxsZXRLaXQgfHwgIWNvbm5lY3RlZCkge1xyXG4gICAgICB0aHJvdyBuZXcgRXJyb3IoJ1dhbGxldCBub3QgY29ubmVjdGVkJyk7XHJcbiAgICB9XHJcbiAgICBcclxuICAgIHRyeSB7XHJcbiAgICAgIGNvbnN0IHJlc3VsdCA9IGF3YWl0IHdhbGxldEtpdC5zaWduVHJhbnNhY3Rpb24oeGRyLCB7XHJcbiAgICAgICAgYWRkcmVzczogd2FsbGV0QWRkcmVzcyxcclxuICAgICAgICBuZXR3b3JrUGFzc3BocmFzZTogXCJUZXN0IFNERiBOZXR3b3JrIDsgU2VwdGVtYmVyIDIwMTVcIiwgLy8gSGFyZGNvZGVkIHRlc3RuZXQgcGFzc3BocmFzZVxyXG4gICAgICB9KTtcclxuICAgICAgcmV0dXJuIHJlc3VsdC5zaWduZWRUeFhkcjtcclxuICAgIH0gY2F0Y2ggKGVycm9yKSB7XHJcbiAgICAgIGNvbnNvbGUuZXJyb3IoJ0Vycm9yIHNpZ25pbmcgdHJhbnNhY3Rpb246JywgZXJyb3IpO1xyXG4gICAgICB0aHJvdyBlcnJvcjtcclxuICAgIH1cclxuICB9O1xyXG5cclxuICBjb25zdCBnZXRQdWJsaWNLZXkgPSAoKTogc3RyaW5nID0+IHtcclxuICAgIGlmICghY29ubmVjdGVkIHx8ICF3YWxsZXRBZGRyZXNzKSB7XHJcbiAgICAgIHRocm93IG5ldyBFcnJvcignV2FsbGV0IG5vdCBjb25uZWN0ZWQnKTtcclxuICAgIH1cclxuICAgIHJldHVybiB3YWxsZXRBZGRyZXNzO1xyXG4gIH07XHJcblxyXG4gIHJldHVybiB7XHJcbiAgICBzaWduVHJhbnNhY3Rpb24sXHJcbiAgICBnZXRQdWJsaWNLZXksXHJcbiAgICBpc1JlYWR5OiBjb25uZWN0ZWQgJiYgd2FsbGV0S2l0ICE9PSBudWxsLFxyXG4gIH07XHJcbn07Il0sIm5hbWVzIjpbIlJlYWN0IiwidXNlQ29udGV4dCIsInVzZVN0YXRlIiwidXNlRWZmZWN0IiwiU3RlbGxhcldhbGxldHNLaXQiLCJXYWxsZXROZXR3b3JrIiwiRnJlaWdodGVyTW9kdWxlIiwiWEJVTExfSUQiLCJ4QnVsbE1vZHVsZSIsIndhbGxldEtpdCIsIm5ldHdvcmsiLCJURVNUTkVUIiwic2VsZWN0ZWRXYWxsZXRJZCIsIm1vZHVsZXMiLCJXYWxsZXRDb250ZXh0IiwiY3JlYXRlQ29udGV4dCIsInVuZGVmaW5lZCIsIldhbGxldFByb3ZpZGVyIiwiY2hpbGRyZW4iLCJjb25uZWN0ZWQiLCJzZXRDb25uZWN0ZWQiLCJsb2FkaW5nIiwic2V0TG9hZGluZyIsIndhbGxldEFkZHJlc3MiLCJzZXRXYWxsZXRBZGRyZXNzIiwiY2hlY2tDb25uZWN0aW9uIiwic3RvcmVkQWRkcmVzcyIsImxvY2FsU3RvcmFnZSIsImdldEl0ZW0iLCJzdG9yZWRXYWxsZXRJZCIsInNldFdhbGxldCIsImFkZHJlc3MiLCJnZXRBZGRyZXNzIiwicmVtb3ZlSXRlbSIsImVycm9yIiwiY29uc29sZSIsImxvZyIsImNvbm5lY3QiLCJvcGVuTW9kYWwiLCJvbldhbGxldFNlbGVjdGVkIiwib3B0aW9uIiwiaWQiLCJzZXRJdGVtIiwib25DbG9zZWQiLCJlcnIiLCJkaXNjb25uZWN0IiwiaGFuZGxlQWNjb3VudENoYW5nZSIsIndpbmRvdyIsImFkZEV2ZW50TGlzdGVuZXIiLCJyZW1vdmVFdmVudExpc3RlbmVyIiwiY29udGV4dFZhbHVlIiwiaXNMb2FkaW5nIiwiUHJvdmlkZXIiLCJ2YWx1ZSIsInVzZVdhbGxldCIsImNvbnRleHQiLCJFcnJvciIsInVzZVdhbGxldE9wZXJhdGlvbnMiLCJzaWduVHJhbnNhY3Rpb24iLCJ4ZHIiLCJyZXN1bHQiLCJuZXR3b3JrUGFzc3BocmFzZSIsInNpZ25lZFR4WGRyIiwiZ2V0UHVibGljS2V5IiwiaXNSZWFkeSJdLCJpZ25vcmVMaXN0IjpbXSwic291cmNlUm9vdCI6IiJ9\n//# sourceURL=webpack-internal:///(pages-dir-node)/./src/contexts/wallet.tsx\n");

/***/ }),

/***/ "(pages-dir-node)/./src/layouts/DefaultLayout.tsx":
/*!***************************************!*\
  !*** ./src/layouts/DefaultLayout.tsx ***!
  \***************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": () => (__WEBPACK_DEFAULT_EXPORT__)\n/* harmony export */ });\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! react/jsx-dev-runtime */ \"react/jsx-dev-runtime\");\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! react */ \"react\");\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_1__);\n/* harmony import */ var _barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! __barrel_optimize__?names=AppBar,Box,Button,Container,Toolbar,Typography!=!@mui/material */ \"(pages-dir-node)/__barrel_optimize__?names=AppBar,Box,Button,Container,Toolbar,Typography!=!./node_modules/@mui/material/esm/index.js\");\n/* harmony import */ var _contexts_wallet__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../contexts/wallet */ \"(pages-dir-node)/./src/contexts/wallet.tsx\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_contexts_wallet__WEBPACK_IMPORTED_MODULE_2__, _barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__]);\n([_contexts_wallet__WEBPACK_IMPORTED_MODULE_2__, _barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\n\nconst DefaultLayout = ({ children })=>{\n    const { connected, walletAddress, connect, disconnect } = (0,_contexts_wallet__WEBPACK_IMPORTED_MODULE_2__.useWallet)();\n    return /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Box, {\n        sx: {\n            flexGrow: 1\n        },\n        children: [\n            /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.AppBar, {\n                position: \"static\",\n                children: /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Toolbar, {\n                    children: [\n                        /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Typography, {\n                            variant: \"h6\",\n                            component: \"div\",\n                            sx: {\n                                flexGrow: 1\n                            },\n                            children: \"YieldBack\"\n                        }, void 0, false, {\n                            fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                            lineNumber: 16,\n                            columnNumber: 11\n                        }, undefined),\n                        connected ? /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Box, {\n                            sx: {\n                                display: 'flex',\n                                alignItems: 'center',\n                                gap: 2\n                            },\n                            children: [\n                                /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Typography, {\n                                    variant: \"body2\",\n                                    children: `${walletAddress.slice(0, 6)}...${walletAddress.slice(-4)}`\n                                }, void 0, false, {\n                                    fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                                    lineNumber: 21,\n                                    columnNumber: 15\n                                }, undefined),\n                                /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Button, {\n                                    color: \"inherit\",\n                                    onClick: disconnect,\n                                    children: \"Disconnect\"\n                                }, void 0, false, {\n                                    fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                                    lineNumber: 24,\n                                    columnNumber: 15\n                                }, undefined)\n                            ]\n                        }, void 0, true, {\n                            fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                            lineNumber: 20,\n                            columnNumber: 13\n                        }, undefined) : /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Button, {\n                            color: \"inherit\",\n                            onClick: connect,\n                            children: \"Connect Wallet\"\n                        }, void 0, false, {\n                            fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                            lineNumber: 29,\n                            columnNumber: 13\n                        }, undefined)\n                    ]\n                }, void 0, true, {\n                    fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                    lineNumber: 15,\n                    columnNumber: 9\n                }, undefined)\n            }, void 0, false, {\n                fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                lineNumber: 14,\n                columnNumber: 7\n            }, undefined),\n            /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_barrel_optimize_names_AppBar_Box_Button_Container_Toolbar_Typography_mui_material__WEBPACK_IMPORTED_MODULE_3__.Container, {\n                maxWidth: \"lg\",\n                sx: {\n                    mt: 4,\n                    mb: 4\n                },\n                children: children\n            }, void 0, false, {\n                fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n                lineNumber: 35,\n                columnNumber: 7\n            }, undefined)\n        ]\n    }, void 0, true, {\n        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\layouts\\\\DefaultLayout.tsx\",\n        lineNumber: 13,\n        columnNumber: 5\n    }, undefined);\n};\n/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (DefaultLayout);\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });//# sourceURL=[module]\n//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHBhZ2VzLWRpci1ub2RlKS8uL3NyYy9sYXlvdXRzL0RlZmF1bHRMYXlvdXQudHN4IiwibWFwcGluZ3MiOiI7Ozs7Ozs7Ozs7Ozs7O0FBQTBCO0FBQzBEO0FBQ3JDO0FBTS9DLE1BQU1RLGdCQUE4QyxDQUFDLEVBQUVDLFFBQVEsRUFBRTtJQUMvRCxNQUFNLEVBQUVDLFNBQVMsRUFBRUMsYUFBYSxFQUFFQyxPQUFPLEVBQUVDLFVBQVUsRUFBRSxHQUFHTiwyREFBU0E7SUFFbkUscUJBQ0UsOERBQUNOLG1IQUFHQTtRQUFDYSxJQUFJO1lBQUVDLFVBQVU7UUFBRTs7MEJBQ3JCLDhEQUFDYixzSEFBTUE7Z0JBQUNjLFVBQVM7MEJBQ2YsNEVBQUNiLHVIQUFPQTs7c0NBQ04sOERBQUNDLDBIQUFVQTs0QkFBQ2EsU0FBUTs0QkFBS0MsV0FBVTs0QkFBTUosSUFBSTtnQ0FBRUMsVUFBVTs0QkFBRTtzQ0FBRzs7Ozs7O3dCQUc3REwsMEJBQ0MsOERBQUNULG1IQUFHQTs0QkFBQ2EsSUFBSTtnQ0FBRUssU0FBUztnQ0FBUUMsWUFBWTtnQ0FBVUMsS0FBSzs0QkFBRTs7OENBQ3ZELDhEQUFDakIsMEhBQVVBO29DQUFDYSxTQUFROzhDQUNqQixHQUFHTixjQUFjVyxLQUFLLENBQUMsR0FBRyxHQUFHLEdBQUcsRUFBRVgsY0FBY1csS0FBSyxDQUFDLENBQUMsSUFBSTs7Ozs7OzhDQUU5RCw4REFBQ2pCLHNIQUFNQTtvQ0FBQ2tCLE9BQU07b0NBQVVDLFNBQVNYOzhDQUFZOzs7Ozs7Ozs7OztzREFLL0MsOERBQUNSLHNIQUFNQTs0QkFBQ2tCLE9BQU07NEJBQVVDLFNBQVNaO3NDQUFTOzs7Ozs7Ozs7Ozs7Ozs7OzswQkFNaEQsOERBQUNOLHlIQUFTQTtnQkFBQ21CLFVBQVM7Z0JBQUtYLElBQUk7b0JBQUVZLElBQUk7b0JBQUdDLElBQUk7Z0JBQUU7MEJBQ3pDbEI7Ozs7Ozs7Ozs7OztBQUlUO0FBRUEsaUVBQWVELGFBQWFBLEVBQUMiLCJzb3VyY2VzIjpbIkM6XFxjb21wc2NpXFxibG9ja2NoYWluXFx5aWVsZGJhY2tcXGZyb250ZW5kXFxzcmNcXGxheW91dHNcXERlZmF1bHRMYXlvdXQudHN4Il0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCBSZWFjdCBmcm9tICdyZWFjdCc7XHJcbmltcG9ydCB7IEJveCwgQXBwQmFyLCBUb29sYmFyLCBUeXBvZ3JhcGh5LCBCdXR0b24sIENvbnRhaW5lciB9IGZyb20gJ0BtdWkvbWF0ZXJpYWwnO1xyXG5pbXBvcnQgeyB1c2VXYWxsZXQgfSBmcm9tICcuLi9jb250ZXh0cy93YWxsZXQnO1xyXG5cclxuaW50ZXJmYWNlIERlZmF1bHRMYXlvdXRQcm9wcyB7XHJcbiAgY2hpbGRyZW46IFJlYWN0LlJlYWN0Tm9kZTtcclxufVxyXG5cclxuY29uc3QgRGVmYXVsdExheW91dDogUmVhY3QuRkM8RGVmYXVsdExheW91dFByb3BzPiA9ICh7IGNoaWxkcmVuIH0pID0+IHtcclxuICBjb25zdCB7IGNvbm5lY3RlZCwgd2FsbGV0QWRkcmVzcywgY29ubmVjdCwgZGlzY29ubmVjdCB9ID0gdXNlV2FsbGV0KCk7XHJcblxyXG4gIHJldHVybiAoXHJcbiAgICA8Qm94IHN4PXt7IGZsZXhHcm93OiAxIH19PlxyXG4gICAgICA8QXBwQmFyIHBvc2l0aW9uPVwic3RhdGljXCI+XHJcbiAgICAgICAgPFRvb2xiYXI+XHJcbiAgICAgICAgICA8VHlwb2dyYXBoeSB2YXJpYW50PVwiaDZcIiBjb21wb25lbnQ9XCJkaXZcIiBzeD17eyBmbGV4R3JvdzogMSB9fT5cclxuICAgICAgICAgICAgWWllbGRCYWNrXHJcbiAgICAgICAgICA8L1R5cG9ncmFwaHk+XHJcbiAgICAgICAgICB7Y29ubmVjdGVkID8gKFxyXG4gICAgICAgICAgICA8Qm94IHN4PXt7IGRpc3BsYXk6ICdmbGV4JywgYWxpZ25JdGVtczogJ2NlbnRlcicsIGdhcDogMiB9fT5cclxuICAgICAgICAgICAgICA8VHlwb2dyYXBoeSB2YXJpYW50PVwiYm9keTJcIj5cclxuICAgICAgICAgICAgICAgIHtgJHt3YWxsZXRBZGRyZXNzLnNsaWNlKDAsIDYpfS4uLiR7d2FsbGV0QWRkcmVzcy5zbGljZSgtNCl9YH1cclxuICAgICAgICAgICAgICA8L1R5cG9ncmFwaHk+XHJcbiAgICAgICAgICAgICAgPEJ1dHRvbiBjb2xvcj1cImluaGVyaXRcIiBvbkNsaWNrPXtkaXNjb25uZWN0fT5cclxuICAgICAgICAgICAgICAgIERpc2Nvbm5lY3RcclxuICAgICAgICAgICAgICA8L0J1dHRvbj5cclxuICAgICAgICAgICAgPC9Cb3g+XHJcbiAgICAgICAgICApIDogKFxyXG4gICAgICAgICAgICA8QnV0dG9uIGNvbG9yPVwiaW5oZXJpdFwiIG9uQ2xpY2s9e2Nvbm5lY3R9PlxyXG4gICAgICAgICAgICAgIENvbm5lY3QgV2FsbGV0XHJcbiAgICAgICAgICAgIDwvQnV0dG9uPlxyXG4gICAgICAgICAgKX1cclxuICAgICAgICA8L1Rvb2xiYXI+XHJcbiAgICAgIDwvQXBwQmFyPlxyXG4gICAgICA8Q29udGFpbmVyIG1heFdpZHRoPVwibGdcIiBzeD17eyBtdDogNCwgbWI6IDQgfX0+XHJcbiAgICAgICAge2NoaWxkcmVufVxyXG4gICAgICA8L0NvbnRhaW5lcj5cclxuICAgIDwvQm94PlxyXG4gICk7XHJcbn07XHJcblxyXG5leHBvcnQgZGVmYXVsdCBEZWZhdWx0TGF5b3V0OyJdLCJuYW1lcyI6WyJSZWFjdCIsIkJveCIsIkFwcEJhciIsIlRvb2xiYXIiLCJUeXBvZ3JhcGh5IiwiQnV0dG9uIiwiQ29udGFpbmVyIiwidXNlV2FsbGV0IiwiRGVmYXVsdExheW91dCIsImNoaWxkcmVuIiwiY29ubmVjdGVkIiwid2FsbGV0QWRkcmVzcyIsImNvbm5lY3QiLCJkaXNjb25uZWN0Iiwic3giLCJmbGV4R3JvdyIsInBvc2l0aW9uIiwidmFyaWFudCIsImNvbXBvbmVudCIsImRpc3BsYXkiLCJhbGlnbkl0ZW1zIiwiZ2FwIiwic2xpY2UiLCJjb2xvciIsIm9uQ2xpY2siLCJtYXhXaWR0aCIsIm10IiwibWIiXSwiaWdub3JlTGlzdCI6W10sInNvdXJjZVJvb3QiOiIifQ==\n//# sourceURL=webpack-internal:///(pages-dir-node)/./src/layouts/DefaultLayout.tsx\n");

/***/ }),

/***/ "(pages-dir-node)/./src/pages/_app.tsx":
/*!****************************!*\
  !*** ./src/pages/_app.tsx ***!
  \****************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": () => (/* binding */ MyApp)\n/* harmony export */ });\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! react/jsx-dev-runtime */ \"react/jsx-dev-runtime\");\n/* harmony import */ var react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var _public_fonts_dm_sans_css__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../../../../../public/fonts/dm-sans.css */ \"(pages-dir-node)/./public/fonts/dm-sans.css\");\n/* harmony import */ var _public_fonts_dm_sans_css__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(_public_fonts_dm_sans_css__WEBPACK_IMPORTED_MODULE_1__);\n/* harmony import */ var _mui_material_CssBaseline__WEBPACK_IMPORTED_MODULE_10__ = __webpack_require__(/*! @mui/material/CssBaseline */ \"(pages-dir-node)/./node_modules/@mui/material/esm/CssBaseline/index.js\");\n/* harmony import */ var _mui_material_styles__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! @mui/material/styles */ \"(pages-dir-node)/./node_modules/@mui/material/esm/styles/index.js\");\n/* harmony import */ var _tanstack_react_query__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @tanstack/react-query */ \"@tanstack/react-query\");\n/* harmony import */ var next_head__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! next/head */ \"(pages-dir-node)/./node_modules/next/head.js\");\n/* harmony import */ var next_head__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(next_head__WEBPACK_IMPORTED_MODULE_3__);\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! react */ \"react\");\n/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_4__);\n/* harmony import */ var _contexts_settings__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ../contexts/settings */ \"(pages-dir-node)/./src/contexts/settings.tsx\");\n/* harmony import */ var _contexts_wallet__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ../contexts/wallet */ \"(pages-dir-node)/./src/contexts/wallet.tsx\");\n/* harmony import */ var _layouts_DefaultLayout__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ../layouts/DefaultLayout */ \"(pages-dir-node)/./src/layouts/DefaultLayout.tsx\");\n/* harmony import */ var _theme_theme__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ../theme/theme */ \"(pages-dir-node)/./src/theme/theme.ts\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_tanstack_react_query__WEBPACK_IMPORTED_MODULE_2__, _contexts_wallet__WEBPACK_IMPORTED_MODULE_6__, _layouts_DefaultLayout__WEBPACK_IMPORTED_MODULE_7__, _theme_theme__WEBPACK_IMPORTED_MODULE_8__, _mui_material_styles__WEBPACK_IMPORTED_MODULE_9__, _mui_material_CssBaseline__WEBPACK_IMPORTED_MODULE_10__]);\n([_tanstack_react_query__WEBPACK_IMPORTED_MODULE_2__, _contexts_wallet__WEBPACK_IMPORTED_MODULE_6__, _layouts_DefaultLayout__WEBPACK_IMPORTED_MODULE_7__, _theme_theme__WEBPACK_IMPORTED_MODULE_8__, _mui_material_styles__WEBPACK_IMPORTED_MODULE_9__, _mui_material_CssBaseline__WEBPACK_IMPORTED_MODULE_10__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\n\n\n\n\n\n\n\n\nfunction MyApp(props) {\n    const { Component, pageProps } = props;\n    const [queryClient] = (0,react__WEBPACK_IMPORTED_MODULE_4__.useState)({\n        \"MyApp.useState\": ()=>new _tanstack_react_query__WEBPACK_IMPORTED_MODULE_2__.QueryClient()\n    }[\"MyApp.useState\"]);\n    return /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.Fragment, {\n        children: [\n            /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)((next_head__WEBPACK_IMPORTED_MODULE_3___default()), {\n                children: [\n                    /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(\"meta\", {\n                        name: \"viewport\",\n                        content: \"initial-scale=1, width=device-width\"\n                    }, void 0, false, {\n                        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                        lineNumber: 21,\n                        columnNumber: 9\n                    }, this),\n                    /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(\"title\", {\n                        children: \"YieldBack - Web3 Fixed Income Protocol\"\n                    }, void 0, false, {\n                        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                        lineNumber: 22,\n                        columnNumber: 9\n                    }, this),\n                    /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(\"meta\", {\n                        name: \"description\",\n                        content: \"Create and manage fixed income positions on Stellar\"\n                    }, void 0, false, {\n                        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                        lineNumber: 23,\n                        columnNumber: 9\n                    }, this)\n                ]\n            }, void 0, true, {\n                fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                lineNumber: 20,\n                columnNumber: 7\n            }, this),\n            /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_tanstack_react_query__WEBPACK_IMPORTED_MODULE_2__.QueryClientProvider, {\n                client: queryClient,\n                children: /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_mui_material_styles__WEBPACK_IMPORTED_MODULE_9__.ThemeProvider, {\n                    theme: _theme_theme__WEBPACK_IMPORTED_MODULE_8__[\"default\"],\n                    children: /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_contexts_settings__WEBPACK_IMPORTED_MODULE_5__.SettingsProvider, {\n                        children: /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_contexts_wallet__WEBPACK_IMPORTED_MODULE_6__.WalletProvider, {\n                            children: [\n                                /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_mui_material_CssBaseline__WEBPACK_IMPORTED_MODULE_10__[\"default\"], {}, void 0, false, {\n                                    fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                                    lineNumber: 29,\n                                    columnNumber: 15\n                                }, this),\n                                /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(_layouts_DefaultLayout__WEBPACK_IMPORTED_MODULE_7__[\"default\"], {\n                                    children: /*#__PURE__*/ (0,react_jsx_dev_runtime__WEBPACK_IMPORTED_MODULE_0__.jsxDEV)(Component, {\n                                        ...pageProps\n                                    }, void 0, false, {\n                                        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                                        lineNumber: 31,\n                                        columnNumber: 17\n                                    }, this)\n                                }, void 0, false, {\n                                    fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                                    lineNumber: 30,\n                                    columnNumber: 15\n                                }, this)\n                            ]\n                        }, void 0, true, {\n                            fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                            lineNumber: 28,\n                            columnNumber: 13\n                        }, this)\n                    }, void 0, false, {\n                        fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                        lineNumber: 27,\n                        columnNumber: 11\n                    }, this)\n                }, void 0, false, {\n                    fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                    lineNumber: 26,\n                    columnNumber: 9\n                }, this)\n            }, void 0, false, {\n                fileName: \"C:\\\\compsci\\\\blockchain\\\\yieldback\\\\frontend\\\\src\\\\pages\\\\_app.tsx\",\n                lineNumber: 25,\n                columnNumber: 7\n            }, this)\n        ]\n    }, void 0, true);\n}\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });//# sourceURL=[module]\n//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHBhZ2VzLWRpci1ub2RlKS8uL3NyYy9wYWdlcy9fYXBwLnRzeCIsIm1hcHBpbmdzIjoiOzs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7OztBQUFtQztBQUVpQjtBQUNDO0FBQ29CO0FBRTVDO0FBQ0k7QUFDdUI7QUFDSjtBQUNDO0FBQ2xCO0FBRXBCLFNBQVNVLE1BQU1DLEtBQWU7SUFDM0MsTUFBTSxFQUFFQyxTQUFTLEVBQUVDLFNBQVMsRUFBRSxHQUFHRjtJQUNqQyxNQUFNLENBQUNHLFlBQVksR0FBR1QsK0NBQVFBOzBCQUFDLElBQU0sSUFBSUgsOERBQVdBOztJQUVwRCxxQkFDRTs7MEJBQ0UsOERBQUNFLGtEQUFJQTs7a0NBQ0gsOERBQUNXO3dCQUFLQyxNQUFLO3dCQUFXQyxTQUFROzs7Ozs7a0NBQzlCLDhEQUFDQztrQ0FBTTs7Ozs7O2tDQUNQLDhEQUFDSDt3QkFBS0MsTUFBSzt3QkFBY0MsU0FBUTs7Ozs7Ozs7Ozs7OzBCQUVuQyw4REFBQ2Qsc0VBQW1CQTtnQkFBQ2dCLFFBQVFMOzBCQUMzQiw0RUFBQ2IsK0RBQWFBO29CQUFDUSxPQUFPQSxvREFBS0E7OEJBQ3pCLDRFQUFDSCxnRUFBZ0JBO2tDQUNmLDRFQUFDQyw0REFBY0E7OzhDQUNiLDhEQUFDUCxrRUFBV0E7Ozs7OzhDQUNaLDhEQUFDUSw4REFBYUE7OENBQ1osNEVBQUNJO3dDQUFXLEdBQUdDLFNBQVM7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7QUFReEMiLCJzb3VyY2VzIjpbIkM6XFxjb21wc2NpXFxibG9ja2NoYWluXFx5aWVsZGJhY2tcXGZyb250ZW5kXFxzcmNcXHBhZ2VzXFxfYXBwLnRzeCJdLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgJy9wdWJsaWMvZm9udHMvZG0tc2Fucy5jc3MnO1xyXG5cclxuaW1wb3J0IENzc0Jhc2VsaW5lIGZyb20gJ0BtdWkvbWF0ZXJpYWwvQ3NzQmFzZWxpbmUnO1xyXG5pbXBvcnQgeyBUaGVtZVByb3ZpZGVyIH0gZnJvbSAnQG11aS9tYXRlcmlhbC9zdHlsZXMnO1xyXG5pbXBvcnQgeyBRdWVyeUNsaWVudCwgUXVlcnlDbGllbnRQcm92aWRlciB9IGZyb20gJ0B0YW5zdGFjay9yZWFjdC1xdWVyeSc7XHJcbmltcG9ydCB7IEFwcFByb3BzIH0gZnJvbSAnbmV4dC9hcHAnO1xyXG5pbXBvcnQgSGVhZCBmcm9tICduZXh0L2hlYWQnO1xyXG5pbXBvcnQgeyB1c2VTdGF0ZSB9IGZyb20gJ3JlYWN0JztcclxuaW1wb3J0IHsgU2V0dGluZ3NQcm92aWRlciB9IGZyb20gJy4uL2NvbnRleHRzL3NldHRpbmdzJztcclxuaW1wb3J0IHsgV2FsbGV0UHJvdmlkZXIgfSBmcm9tICcuLi9jb250ZXh0cy93YWxsZXQnO1xyXG5pbXBvcnQgRGVmYXVsdExheW91dCBmcm9tICcuLi9sYXlvdXRzL0RlZmF1bHRMYXlvdXQnO1xyXG5pbXBvcnQgdGhlbWUgZnJvbSAnLi4vdGhlbWUvdGhlbWUnO1xyXG5cclxuZXhwb3J0IGRlZmF1bHQgZnVuY3Rpb24gTXlBcHAocHJvcHM6IEFwcFByb3BzKSB7XHJcbiAgY29uc3QgeyBDb21wb25lbnQsIHBhZ2VQcm9wcyB9ID0gcHJvcHM7XHJcbiAgY29uc3QgW3F1ZXJ5Q2xpZW50XSA9IHVzZVN0YXRlKCgpID0+IG5ldyBRdWVyeUNsaWVudCgpKTtcclxuXHJcbiAgcmV0dXJuIChcclxuICAgIDw+XHJcbiAgICAgIDxIZWFkPlxyXG4gICAgICAgIDxtZXRhIG5hbWU9XCJ2aWV3cG9ydFwiIGNvbnRlbnQ9XCJpbml0aWFsLXNjYWxlPTEsIHdpZHRoPWRldmljZS13aWR0aFwiIC8+XHJcbiAgICAgICAgPHRpdGxlPllpZWxkQmFjayAtIFdlYjMgRml4ZWQgSW5jb21lIFByb3RvY29sPC90aXRsZT5cclxuICAgICAgICA8bWV0YSBuYW1lPVwiZGVzY3JpcHRpb25cIiBjb250ZW50PVwiQ3JlYXRlIGFuZCBtYW5hZ2UgZml4ZWQgaW5jb21lIHBvc2l0aW9ucyBvbiBTdGVsbGFyXCIgLz5cclxuICAgICAgPC9IZWFkPlxyXG4gICAgICA8UXVlcnlDbGllbnRQcm92aWRlciBjbGllbnQ9e3F1ZXJ5Q2xpZW50fT5cclxuICAgICAgICA8VGhlbWVQcm92aWRlciB0aGVtZT17dGhlbWV9PlxyXG4gICAgICAgICAgPFNldHRpbmdzUHJvdmlkZXI+XHJcbiAgICAgICAgICAgIDxXYWxsZXRQcm92aWRlcj5cclxuICAgICAgICAgICAgICA8Q3NzQmFzZWxpbmUgLz5cclxuICAgICAgICAgICAgICA8RGVmYXVsdExheW91dD5cclxuICAgICAgICAgICAgICAgIDxDb21wb25lbnQgey4uLnBhZ2VQcm9wc30gLz5cclxuICAgICAgICAgICAgICA8L0RlZmF1bHRMYXlvdXQ+XHJcbiAgICAgICAgICAgIDwvV2FsbGV0UHJvdmlkZXI+XHJcbiAgICAgICAgICA8L1NldHRpbmdzUHJvdmlkZXI+XHJcbiAgICAgICAgPC9UaGVtZVByb3ZpZGVyPlxyXG4gICAgICA8L1F1ZXJ5Q2xpZW50UHJvdmlkZXI+XHJcbiAgICA8Lz5cclxuICApO1xyXG59Il0sIm5hbWVzIjpbIkNzc0Jhc2VsaW5lIiwiVGhlbWVQcm92aWRlciIsIlF1ZXJ5Q2xpZW50IiwiUXVlcnlDbGllbnRQcm92aWRlciIsIkhlYWQiLCJ1c2VTdGF0ZSIsIlNldHRpbmdzUHJvdmlkZXIiLCJXYWxsZXRQcm92aWRlciIsIkRlZmF1bHRMYXlvdXQiLCJ0aGVtZSIsIk15QXBwIiwicHJvcHMiLCJDb21wb25lbnQiLCJwYWdlUHJvcHMiLCJxdWVyeUNsaWVudCIsIm1ldGEiLCJuYW1lIiwiY29udGVudCIsInRpdGxlIiwiY2xpZW50Il0sImlnbm9yZUxpc3QiOltdLCJzb3VyY2VSb290IjoiIn0=\n//# sourceURL=webpack-internal:///(pages-dir-node)/./src/pages/_app.tsx\n");

/***/ }),

/***/ "(pages-dir-node)/./src/theme/theme.ts":
/*!****************************!*\
  !*** ./src/theme/theme.ts ***!
  \****************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": () => (__WEBPACK_DEFAULT_EXPORT__)\n/* harmony export */ });\n/* harmony import */ var _mui_material_styles__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @mui/material/styles */ \"(pages-dir-node)/./node_modules/@mui/material/esm/styles/index.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_mui_material_styles__WEBPACK_IMPORTED_MODULE_0__]);\n_mui_material_styles__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n// Define your brand colors\nconst colors = {\n    primary: {\n        main: '#2563eb',\n        light: '#3b82f6',\n        dark: '#1d4ed8',\n        contrastText: '#ffffff'\n    },\n    secondary: {\n        main: '#10b981',\n        light: '#34d399',\n        dark: '#059669',\n        contrastText: '#ffffff'\n    },\n    background: {\n        default: '#f8fafc',\n        paper: '#ffffff'\n    },\n    text: {\n        primary: '#1e293b',\n        secondary: '#64748b'\n    }\n};\nconst theme = (0,_mui_material_styles__WEBPACK_IMPORTED_MODULE_0__.createTheme)({\n    palette: {\n        primary: colors.primary,\n        secondary: colors.secondary,\n        background: colors.background,\n        text: colors.text,\n        success: {\n            main: '#10b981',\n            light: '#34d399',\n            dark: '#059669'\n        },\n        error: {\n            main: '#ef4444',\n            light: '#f87171',\n            dark: '#dc2626'\n        },\n        warning: {\n            main: '#f59e0b',\n            light: '#fbbf24',\n            dark: '#d97706'\n        },\n        info: {\n            main: colors.primary.main\n        }\n    },\n    typography: {\n        fontFamily: '\"DM Sans\", \"Inter\", \"Roboto\", \"Helvetica\", \"Arial\", sans-serif',\n        h1: {\n            fontSize: '2.5rem',\n            fontWeight: 700,\n            lineHeight: 1.2\n        },\n        h2: {\n            fontSize: '2rem',\n            fontWeight: 600,\n            lineHeight: 1.3\n        },\n        h3: {\n            fontSize: '1.75rem',\n            fontWeight: 600,\n            lineHeight: 1.3\n        },\n        h4: {\n            fontSize: '1.5rem',\n            fontWeight: 600,\n            lineHeight: 1.4\n        },\n        h5: {\n            fontSize: '1.25rem',\n            fontWeight: 600,\n            lineHeight: 1.4\n        },\n        h6: {\n            fontSize: '1.125rem',\n            fontWeight: 600,\n            lineHeight: 1.4\n        },\n        body1: {\n            fontSize: '1rem',\n            lineHeight: 1.6\n        },\n        body2: {\n            fontSize: '0.875rem',\n            lineHeight: 1.6\n        },\n        button: {\n            textTransform: 'none',\n            fontWeight: 500\n        }\n    },\n    shape: {\n        borderRadius: 8\n    },\n    spacing: 8,\n    components: {\n        MuiButton: {\n            styleOverrides: {\n                root: {\n                    textTransform: 'none',\n                    fontWeight: 500,\n                    borderRadius: 8,\n                    padding: '8px 16px'\n                },\n                contained: {\n                    boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)',\n                    '&:hover': {\n                        boxShadow: '0 4px 8px rgba(0, 0, 0, 0.15)'\n                    }\n                }\n            }\n        },\n        MuiCard: {\n            styleOverrides: {\n                root: {\n                    boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06)',\n                    border: '1px solid #e2e8f0',\n                    '&:hover': {\n                        boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06)'\n                    }\n                }\n            }\n        },\n        MuiTextField: {\n            styleOverrides: {\n                root: {\n                    '& .MuiOutlinedInput-root': {\n                        backgroundColor: '#ffffff',\n                        '&:hover .MuiOutlinedInput-notchedOutline': {\n                            borderColor: colors.primary.main\n                        }\n                    }\n                }\n            }\n        },\n        MuiAppBar: {\n            styleOverrides: {\n                root: {\n                    backgroundColor: '#ffffff',\n                    color: colors.text.primary,\n                    boxShadow: '0 1px 2px rgba(0, 0, 0, 0.05)',\n                    borderBottom: '1px solid #e2e8f0'\n                }\n            }\n        },\n        MuiAlert: {\n            styleOverrides: {\n                root: {\n                    borderRadius: 8\n                }\n            }\n        }\n    }\n});\n/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (theme);\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });//# sourceURL=[module]\n//# sourceMappingURL=data:application/json;charset=utf-8;base64,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b21wc2NpXFxibG9ja2NoYWluXFx5aWVsZGJhY2tcXGZyb250ZW5kXFxzcmNcXHRoZW1lXFx0aGVtZS50cyJdLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgeyBjcmVhdGVUaGVtZSB9IGZyb20gJ0BtdWkvbWF0ZXJpYWwvc3R5bGVzJztcclxuXHJcbi8vIERlZmluZSB5b3VyIGJyYW5kIGNvbG9yc1xyXG5jb25zdCBjb2xvcnMgPSB7XHJcbiAgcHJpbWFyeToge1xyXG4gICAgbWFpbjogJyMyNTYzZWInLCAvLyBCbHVlXHJcbiAgICBsaWdodDogJyMzYjgyZjYnLFxyXG4gICAgZGFyazogJyMxZDRlZDgnLFxyXG4gICAgY29udHJhc3RUZXh0OiAnI2ZmZmZmZicsXHJcbiAgfSxcclxuICBzZWNvbmRhcnk6IHtcclxuICAgIG1haW46ICcjMTBiOTgxJywgLy8gR3JlZW4gZm9yIHN1Y2Nlc3MvcHJvZml0XHJcbiAgICBsaWdodDogJyMzNGQzOTknLFxyXG4gICAgZGFyazogJyMwNTk2NjknLFxyXG4gICAgY29udHJhc3RUZXh0OiAnI2ZmZmZmZicsXHJcbiAgfSxcclxuICBiYWNrZ3JvdW5kOiB7XHJcbiAgICBkZWZhdWx0OiAnI2Y4ZmFmYycsIC8vIFZlcnkgbGlnaHQgZ3JheVxyXG4gICAgcGFwZXI6ICcjZmZmZmZmJyxcclxuICB9LFxyXG4gIHRleHQ6IHtcclxuICAgIHByaW1hcnk6ICcjMWUyOTNiJyxcclxuICAgIHNlY29uZGFyeTogJyM2NDc0OGInLFxyXG4gIH0sXHJcbn07XHJcblxyXG5jb25zdCB0aGVtZSA9IGNyZWF0ZVRoZW1lKHtcclxuICBwYWxldHRlOiB7XHJcbiAgICBwcmltYXJ5OiBjb2xvcnMucHJpbWFyeSxcclxuICAgIHNlY29uZGFyeTogY29sb3JzLnNlY29uZGFyeSxcclxuICAgIGJhY2tncm91bmQ6IGNvbG9ycy5iYWNrZ3JvdW5kLFxyXG4gICAgdGV4dDogY29sb3JzLnRleHQsXHJcbiAgICBzdWNjZXNzOiB7XHJcbiAgICAgIG1haW46ICcjMTBiOTgxJyxcclxuICAgICAgbGlnaHQ6ICcjMzRkMzk5JyxcclxuICAgICAgZGFyazogJyMwNTk2NjknLFxyXG4gICAgfSxcclxuICAgIGVycm9yOiB7XHJcbiAgICAgIG1haW46ICcjZWY0NDQ0JyxcclxuICAgICAgbGlnaHQ6ICcjZjg3MTcxJyxcclxuICAgICAgZGFyazogJyNkYzI2MjYnLFxyXG4gICAgfSxcclxuICAgIHdhcm5pbmc6IHtcclxuICAgICAgbWFpbjogJyNmNTllMGInLFxyXG4gICAgICBsaWdodDogJyNmYmJmMjQnLFxyXG4gICAgICBkYXJrOiAnI2Q5NzcwNicsXHJcbiAgICB9LFxyXG4gICAgaW5mbzoge1xyXG4gICAgICBtYWluOiBjb2xvcnMucHJpbWFyeS5tYWluLFxyXG4gICAgfSxcclxuICB9LFxyXG4gIHR5cG9ncmFwaHk6IHtcclxuICAgIGZvbnRGYW1pbHk6ICdcIkRNIFNhbnNcIiwgXCJJbnRlclwiLCBcIlJvYm90b1wiLCBcIkhlbHZldGljYVwiLCBcIkFyaWFsXCIsIHNhbnMtc2VyaWYnLFxyXG4gICAgaDE6IHtcclxuICAgICAgZm9udFNpemU6ICcyLjVyZW0nLFxyXG4gICAgICBmb250V2VpZ2h0OiA3MDAsXHJcbiAgICAgIGxpbmVIZWlnaHQ6IDEuMixcclxuICAgIH0sXHJcbiAgICBoMjoge1xyXG4gICAgICBmb250U2l6ZTogJzJyZW0nLFxyXG4gICAgICBmb250V2VpZ2h0OiA2MDAsXHJcbiAgICAgIGxpbmVIZWlnaHQ6IDEuMyxcclxuICAgIH0sXHJcbiAgICBoMzoge1xyXG4gICAgICBmb250U2l6ZTogJzEuNzVyZW0nLFxyXG4gICAgICBmb250V2VpZ2h0OiA2MDAsXHJcbiAgICAgIGxpbmVIZWlnaHQ6IDEuMyxcclxuICAgIH0sXHJcbiAgICBoNDoge1xyXG4gICAgICBmb250U2l6ZTogJzEuNXJlbScsXHJcbiAgICAgIGZvbnRXZWlnaHQ6IDYwMCxcclxuICAgICAgbGluZUhlaWdodDogMS40LFxyXG4gICAgfSxcclxuICAgIGg1OiB7XHJcbiAgICAgIGZvbnRTaXplOiAnMS4yNXJlbScsXHJcbiAgICAgIGZvbnRXZWlnaHQ6IDYwMCxcclxuICAgICAgbGluZUhlaWdodDogMS40LFxyXG4gICAgfSxcclxuICAgIGg2OiB7XHJcbiAgICAgIGZvbnRTaXplOiAnMS4xMjVyZW0nLFxyXG4gICAgICBmb250V2VpZ2h0OiA2MDAsXHJcbiAgICAgIGxpbmVIZWlnaHQ6IDEuNCxcclxuICAgIH0sXHJcbiAgICBib2R5MToge1xyXG4gICAgICBmb250U2l6ZTogJzFyZW0nLFxyXG4gICAgICBsaW5lSGVpZ2h0OiAxLjYsXHJcbiAgICB9LFxyXG4gICAgYm9keTI6IHtcclxuICAgICAgZm9udFNpemU6ICcwLjg3NXJlbScsXHJcbiAgICAgIGxpbmVIZWlnaHQ6IDEuNixcclxuICAgIH0sXHJcbiAgICBidXR0b246IHtcclxuICAgICAgdGV4dFRyYW5zZm9ybTogJ25vbmUnLFxyXG4gICAgICBmb250V2VpZ2h0OiA1MDAsXHJcbiAgICB9LFxyXG4gIH0sXHJcbiAgc2hhcGU6IHtcclxuICAgIGJvcmRlclJhZGl1czogOCxcclxuICB9LFxyXG4gIHNwYWNpbmc6IDgsXHJcbiAgY29tcG9uZW50czoge1xyXG4gICAgTXVpQnV0dG9uOiB7XHJcbiAgICAgIHN0eWxlT3ZlcnJpZGVzOiB7XHJcbiAgICAgICAgcm9vdDoge1xyXG4gICAgICAgICAgdGV4dFRyYW5zZm9ybTogJ25vbmUnLFxyXG4gICAgICAgICAgZm9udFdlaWdodDogNTAwLFxyXG4gICAgICAgICAgYm9yZGVyUmFkaXVzOiA4LFxyXG4gICAgICAgICAgcGFkZGluZzogJzhweCAxNnB4JyxcclxuICAgICAgICB9LFxyXG4gICAgICAgIGNvbnRhaW5lZDoge1xyXG4gICAgICAgICAgYm94U2hhZG93OiAnMCAycHggNHB4IHJnYmEoMCwgMCwgMCwgMC4xKScsXHJcbiAgICAgICAgICAnJjpob3Zlcic6IHtcclxuICAgICAgICAgICAgYm94U2hhZG93OiAnMCA0cHggOHB4IHJnYmEoMCwgMCwgMCwgMC4xNSknLFxyXG4gICAgICAgICAgfSxcclxuICAgICAgICB9LFxyXG4gICAgICB9LFxyXG4gICAgfSxcclxuICAgIE11aUNhcmQ6IHtcclxuICAgICAgc3R5bGVPdmVycmlkZXM6IHtcclxuICAgICAgICByb290OiB7XHJcbiAgICAgICAgICBib3hTaGFkb3c6ICcwIDFweCAzcHggcmdiYSgwLCAwLCAwLCAwLjEpLCAwIDFweCAycHggcmdiYSgwLCAwLCAwLCAwLjA2KScsXHJcbiAgICAgICAgICBib3JkZXI6ICcxcHggc29saWQgI2UyZThmMCcsXHJcbiAgICAgICAgICAnJjpob3Zlcic6IHtcclxuICAgICAgICAgICAgYm94U2hhZG93OiAnMCA0cHggNnB4IHJnYmEoMCwgMCwgMCwgMC4xKSwgMCAycHggNHB4IHJnYmEoMCwgMCwgMCwgMC4wNiknLFxyXG4gICAgICAgICAgfSxcclxuICAgICAgICB9LFxyXG4gICAgICB9LFxyXG4gICAgfSxcclxuICAgIE11aVRleHRGaWVsZDoge1xyXG4gICAgICBzdHlsZU92ZXJyaWRlczoge1xyXG4gICAgICAgIHJvb3Q6IHtcclxuICAgICAgICAgICcmIC5NdWlPdXRsaW5lZElucHV0LXJvb3QnOiB7XHJcbiAgICAgICAgICAgIGJhY2tncm91bmRDb2xvcjogJyNmZmZmZmYnLFxyXG4gICAgICAgICAgICAnJjpob3ZlciAuTXVpT3V0bGluZWRJbnB1dC1ub3RjaGVkT3V0bGluZSc6IHtcclxuICAgICAgICAgICAgICBib3JkZXJDb2xvcjogY29sb3JzLnByaW1hcnkubWFpbixcclxuICAgICAgICAgICAgfSxcclxuICAgICAgICAgIH0sXHJcbiAgICAgICAgfSxcclxuICAgICAgfSxcclxuICAgIH0sXHJcbiAgICBNdWlBcHBCYXI6IHtcclxuICAgICAgc3R5bGVPdmVycmlkZXM6IHtcclxuICAgICAgICByb290OiB7XHJcbiAgICAgICAgICBiYWNrZ3JvdW5kQ29sb3I6ICcjZmZmZmZmJyxcclxuICAgICAgICAgIGNvbG9yOiBjb2xvcnMudGV4dC5wcmltYXJ5LFxyXG4gICAgICAgICAgYm94U2hhZG93OiAnMCAxcHggMnB4IHJnYmEoMCwgMCwgMCwgMC4wNSknLFxyXG4gICAgICAgICAgYm9yZGVyQm90dG9tOiAnMXB4IHNvbGlkICNlMmU4ZjAnLFxyXG4gICAgICAgIH0sXHJcbiAgICAgIH0sXHJcbiAgICB9LFxyXG4gICAgTXVpQWxlcnQ6IHtcclxuICAgICAgc3R5bGVPdmVycmlkZXM6IHtcclxuICAgICAgICByb290OiB7XHJcbiAgICAgICAgICBib3JkZXJSYWRpdXM6IDgsXHJcbiAgICAgICAgfSxcclxuICAgICAgfSxcclxuICAgIH0sXHJcbiAgfSxcclxufSk7XHJcblxyXG5leHBvcnQgZGVmYXVsdCB0aGVtZTsiXSwibmFtZXMiOlsiY3JlYXRlVGhlbWUiLCJjb2xvcnMiLCJwcmltYXJ5IiwibWFpbiIsImxpZ2h0IiwiZGFyayIsImNvbnRyYXN0VGV4dCIsInNlY29uZGFyeSIsImJhY2tncm91bmQiLCJkZWZhdWx0IiwicGFwZXIiLCJ0ZXh0IiwidGhlbWUiLCJwYWxldHRlIiwic3VjY2VzcyIsImVycm9yIiwid2FybmluZyIsImluZm8iLCJ0eXBvZ3JhcGh5IiwiZm9udEZhbWlseSIsImgxIiwiZm9udFNpemUiLCJmb250V2VpZ2h0IiwibGluZUhlaWdodCIsImgyIiwiaDMiLCJoNCIsImg1IiwiaDYiLCJib2R5MSIsImJvZHkyIiwiYnV0dG9uIiwidGV4dFRyYW5zZm9ybSIsInNoYXBlIiwiYm9yZGVyUmFkaXVzIiwic3BhY2luZyIsImNvbXBvbmVudHMiLCJNdWlCdXR0b24iLCJzdHlsZU92ZXJyaWRlcyIsInJvb3QiLCJwYWRkaW5nIiwiY29udGFpbmVkIiwiYm94U2hhZG93IiwiTXVpQ2FyZCIsImJvcmRlciIsIk11aVRleHRGaWVsZCIsImJhY2tncm91bmRDb2xvciIsImJvcmRlckNvbG9yIiwiTXVpQXBwQmFyIiwiY29sb3IiLCJib3JkZXJCb3R0b20iLCJNdWlBbGVydCJdLCJpZ25vcmVMaXN0IjpbXSwic291cmNlUm9vdCI6IiJ9\n//# sourceURL=webpack-internal:///(pages-dir-node)/./src/theme/theme.ts\n");

/***/ }),

/***/ "(pages-dir-node)/__barrel_optimize__?names=AppBar,Box,Button,Container,Toolbar,Typography!=!./node_modules/@mui/material/esm/index.js":
/*!****************************************************************************************************************************!*\
  !*** __barrel_optimize__?names=AppBar,Box,Button,Container,Toolbar,Typography!=!./node_modules/@mui/material/esm/index.js ***!
  \****************************************************************************************************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   AppBar: () => (/* reexport safe */ _AppBar_index_js__WEBPACK_IMPORTED_MODULE_0__[\"default\"]),\n/* harmony export */   Box: () => (/* reexport safe */ _Box_index_js__WEBPACK_IMPORTED_MODULE_1__[\"default\"]),\n/* harmony export */   Button: () => (/* reexport safe */ _Button_index_js__WEBPACK_IMPORTED_MODULE_2__[\"default\"]),\n/* harmony export */   Container: () => (/* reexport safe */ _Container_index_js__WEBPACK_IMPORTED_MODULE_3__[\"default\"]),\n/* harmony export */   Toolbar: () => (/* reexport safe */ _Toolbar_index_js__WEBPACK_IMPORTED_MODULE_4__[\"default\"]),\n/* harmony export */   Typography: () => (/* reexport safe */ _Typography_index_js__WEBPACK_IMPORTED_MODULE_5__[\"default\"])\n/* harmony export */ });\n/* harmony import */ var _AppBar_index_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./AppBar/index.js */ \"(pages-dir-node)/./node_modules/@mui/material/esm/AppBar/index.js\");\n/* harmony import */ var _Box_index_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Box/index.js */ \"(pages-dir-node)/./node_modules/@mui/material/esm/Box/index.js\");\n/* harmony import */ var _Button_index_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./Button/index.js */ \"(pages-dir-node)/./node_modules/@mui/material/esm/Button/index.js\");\n/* harmony import */ var _Container_index_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./Container/index.js */ \"(pages-dir-node)/./node_modules/@mui/material/esm/Container/index.js\");\n/* harmony import */ var _Toolbar_index_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./Toolbar/index.js */ \"(pages-dir-node)/./node_modules/@mui/material/esm/Toolbar/index.js\");\n/* harmony import */ var _Typography_index_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./Typography/index.js */ \"(pages-dir-node)/./node_modules/@mui/material/esm/Typography/index.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_AppBar_index_js__WEBPACK_IMPORTED_MODULE_0__, _Box_index_js__WEBPACK_IMPORTED_MODULE_1__, _Button_index_js__WEBPACK_IMPORTED_MODULE_2__, _Container_index_js__WEBPACK_IMPORTED_MODULE_3__, _Toolbar_index_js__WEBPACK_IMPORTED_MODULE_4__, _Typography_index_js__WEBPACK_IMPORTED_MODULE_5__]);\n([_AppBar_index_js__WEBPACK_IMPORTED_MODULE_0__, _Box_index_js__WEBPACK_IMPORTED_MODULE_1__, _Button_index_js__WEBPACK_IMPORTED_MODULE_2__, _Container_index_js__WEBPACK_IMPORTED_MODULE_3__, _Toolbar_index_js__WEBPACK_IMPORTED_MODULE_4__, _Typography_index_js__WEBPACK_IMPORTED_MODULE_5__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\n\n\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });//# sourceURL=[module]\n//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHBhZ2VzLWRpci1ub2RlKS9fX2JhcnJlbF9vcHRpbWl6ZV9fP25hbWVzPUFwcEJhcixCb3gsQnV0dG9uLENvbnRhaW5lcixUb29sYmFyLFR5cG9ncmFwaHkhPSEuL25vZGVfbW9kdWxlcy9AbXVpL21hdGVyaWFsL2VzbS9pbmRleC5qcyIsIm1hcHBpbmdzIjoiOzs7Ozs7Ozs7Ozs7Ozs7Ozs7O0FBQ3FEO0FBQ047QUFDTTtBQUNNO0FBQ0oiLCJzb3VyY2VzIjpbIkM6XFxjb21wc2NpXFxibG9ja2NoYWluXFx5aWVsZGJhY2tcXGZyb250ZW5kXFxub2RlX21vZHVsZXNcXEBtdWlcXG1hdGVyaWFsXFxlc21cXGluZGV4LmpzIl0sInNvdXJjZXNDb250ZW50IjpbIlxuZXhwb3J0IHsgZGVmYXVsdCBhcyBBcHBCYXIgfSBmcm9tIFwiLi9BcHBCYXIvaW5kZXguanNcIlxuZXhwb3J0IHsgZGVmYXVsdCBhcyBCb3ggfSBmcm9tIFwiLi9Cb3gvaW5kZXguanNcIlxuZXhwb3J0IHsgZGVmYXVsdCBhcyBCdXR0b24gfSBmcm9tIFwiLi9CdXR0b24vaW5kZXguanNcIlxuZXhwb3J0IHsgZGVmYXVsdCBhcyBDb250YWluZXIgfSBmcm9tIFwiLi9Db250YWluZXIvaW5kZXguanNcIlxuZXhwb3J0IHsgZGVmYXVsdCBhcyBUb29sYmFyIH0gZnJvbSBcIi4vVG9vbGJhci9pbmRleC5qc1wiXG5leHBvcnQgeyBkZWZhdWx0IGFzIFR5cG9ncmFwaHkgfSBmcm9tIFwiLi9UeXBvZ3JhcGh5L2luZGV4LmpzXCIiXSwibmFtZXMiOltdLCJpZ25vcmVMaXN0IjpbMF0sInNvdXJjZVJvb3QiOiIifQ==\n//# sourceURL=webpack-internal:///(pages-dir-node)/__barrel_optimize__?names=AppBar,Box,Button,Container,Toolbar,Typography!=!./node_modules/@mui/material/esm/index.js\n");

/***/ }),

/***/ "@creit.tech/stellar-wallets-kit":
/*!**************************************************!*\
  !*** external "@creit.tech/stellar-wallets-kit" ***!
  \**************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@creit.tech/stellar-wallets-kit");;

/***/ }),

/***/ "@mui/system":
/*!******************************!*\
  !*** external "@mui/system" ***!
  \******************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system");;

/***/ }),

/***/ "@mui/system/DefaultPropsProvider":
/*!***************************************************!*\
  !*** external "@mui/system/DefaultPropsProvider" ***!
  \***************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/DefaultPropsProvider");;

/***/ }),

/***/ "@mui/system/InitColorSchemeScript":
/*!****************************************************!*\
  !*** external "@mui/system/InitColorSchemeScript" ***!
  \****************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/InitColorSchemeScript");;

/***/ }),

/***/ "@mui/system/colorManipulator":
/*!***********************************************!*\
  !*** external "@mui/system/colorManipulator" ***!
  \***********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/colorManipulator");;

/***/ }),

/***/ "@mui/system/createBreakpoints":
/*!************************************************!*\
  !*** external "@mui/system/createBreakpoints" ***!
  \************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/createBreakpoints");;

/***/ }),

/***/ "@mui/system/createStyled":
/*!*******************************************!*\
  !*** external "@mui/system/createStyled" ***!
  \*******************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/createStyled");;

/***/ }),

/***/ "@mui/system/createTheme":
/*!******************************************!*\
  !*** external "@mui/system/createTheme" ***!
  \******************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/createTheme");;

/***/ }),

/***/ "@mui/system/cssVars":
/*!**************************************!*\
  !*** external "@mui/system/cssVars" ***!
  \**************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/cssVars");;

/***/ }),

/***/ "@mui/system/spacing":
/*!**************************************!*\
  !*** external "@mui/system/spacing" ***!
  \**************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/spacing");;

/***/ }),

/***/ "@mui/system/styleFunctionSx":
/*!**********************************************!*\
  !*** external "@mui/system/styleFunctionSx" ***!
  \**********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/styleFunctionSx");;

/***/ }),

/***/ "@mui/system/useThemeProps":
/*!********************************************!*\
  !*** external "@mui/system/useThemeProps" ***!
  \********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/system/useThemeProps");;

/***/ }),

/***/ "@mui/utils/ClassNameGenerator":
/*!************************************************!*\
  !*** external "@mui/utils/ClassNameGenerator" ***!
  \************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/ClassNameGenerator");;

/***/ }),

/***/ "@mui/utils/capitalize":
/*!****************************************!*\
  !*** external "@mui/utils/capitalize" ***!
  \****************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/capitalize");;

/***/ }),

/***/ "@mui/utils/chainPropTypes":
/*!********************************************!*\
  !*** external "@mui/utils/chainPropTypes" ***!
  \********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/chainPropTypes");;

/***/ }),

/***/ "@mui/utils/composeClasses":
/*!********************************************!*\
  !*** external "@mui/utils/composeClasses" ***!
  \********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/composeClasses");;

/***/ }),

/***/ "@mui/utils/createChainedFunction":
/*!***************************************************!*\
  !*** external "@mui/utils/createChainedFunction" ***!
  \***************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/createChainedFunction");;

/***/ }),

/***/ "@mui/utils/debounce":
/*!**************************************!*\
  !*** external "@mui/utils/debounce" ***!
  \**************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/debounce");;

/***/ }),

/***/ "@mui/utils/deepmerge":
/*!***************************************!*\
  !*** external "@mui/utils/deepmerge" ***!
  \***************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/deepmerge");;

/***/ }),

/***/ "@mui/utils/deprecatedPropType":
/*!************************************************!*\
  !*** external "@mui/utils/deprecatedPropType" ***!
  \************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/deprecatedPropType");;

/***/ }),

/***/ "@mui/utils/elementTypeAcceptingRef":
/*!*****************************************************!*\
  !*** external "@mui/utils/elementTypeAcceptingRef" ***!
  \*****************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/elementTypeAcceptingRef");;

/***/ }),

/***/ "@mui/utils/formatMuiErrorMessage":
/*!***************************************************!*\
  !*** external "@mui/utils/formatMuiErrorMessage" ***!
  \***************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/formatMuiErrorMessage");;

/***/ }),

/***/ "@mui/utils/generateUtilityClass":
/*!**************************************************!*\
  !*** external "@mui/utils/generateUtilityClass" ***!
  \**************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/generateUtilityClass");;

/***/ }),

/***/ "@mui/utils/generateUtilityClasses":
/*!****************************************************!*\
  !*** external "@mui/utils/generateUtilityClasses" ***!
  \****************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/generateUtilityClasses");;

/***/ }),

/***/ "@mui/utils/integerPropType":
/*!*********************************************!*\
  !*** external "@mui/utils/integerPropType" ***!
  \*********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/integerPropType");;

/***/ }),

/***/ "@mui/utils/isFocusVisible":
/*!********************************************!*\
  !*** external "@mui/utils/isFocusVisible" ***!
  \********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/isFocusVisible");;

/***/ }),

/***/ "@mui/utils/isMuiElement":
/*!******************************************!*\
  !*** external "@mui/utils/isMuiElement" ***!
  \******************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/isMuiElement");;

/***/ }),

/***/ "@mui/utils/ownerDocument":
/*!*******************************************!*\
  !*** external "@mui/utils/ownerDocument" ***!
  \*******************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/ownerDocument");;

/***/ }),

/***/ "@mui/utils/ownerWindow":
/*!*****************************************!*\
  !*** external "@mui/utils/ownerWindow" ***!
  \*****************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/ownerWindow");;

/***/ }),

/***/ "@mui/utils/refType":
/*!*************************************!*\
  !*** external "@mui/utils/refType" ***!
  \*************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/refType");;

/***/ }),

/***/ "@mui/utils/requirePropFactory":
/*!************************************************!*\
  !*** external "@mui/utils/requirePropFactory" ***!
  \************************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/requirePropFactory");;

/***/ }),

/***/ "@mui/utils/resolveProps":
/*!******************************************!*\
  !*** external "@mui/utils/resolveProps" ***!
  \******************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/resolveProps");;

/***/ }),

/***/ "@mui/utils/setRef":
/*!************************************!*\
  !*** external "@mui/utils/setRef" ***!
  \************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/setRef");;

/***/ }),

/***/ "@mui/utils/unsupportedProp":
/*!*********************************************!*\
  !*** external "@mui/utils/unsupportedProp" ***!
  \*********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/unsupportedProp");;

/***/ }),

/***/ "@mui/utils/useControlled":
/*!*******************************************!*\
  !*** external "@mui/utils/useControlled" ***!
  \*******************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/useControlled");;

/***/ }),

/***/ "@mui/utils/useEnhancedEffect":
/*!***********************************************!*\
  !*** external "@mui/utils/useEnhancedEffect" ***!
  \***********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/useEnhancedEffect");;

/***/ }),

/***/ "@mui/utils/useEventCallback":
/*!**********************************************!*\
  !*** external "@mui/utils/useEventCallback" ***!
  \**********************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/useEventCallback");;

/***/ }),

/***/ "@mui/utils/useForkRef":
/*!****************************************!*\
  !*** external "@mui/utils/useForkRef" ***!
  \****************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/useForkRef");;

/***/ }),

/***/ "@mui/utils/useId":
/*!***********************************!*\
  !*** external "@mui/utils/useId" ***!
  \***********************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/useId");;

/***/ }),

/***/ "@mui/utils/useLazyRef":
/*!****************************************!*\
  !*** external "@mui/utils/useLazyRef" ***!
  \****************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/useLazyRef");;

/***/ }),

/***/ "@mui/utils/useTimeout":
/*!****************************************!*\
  !*** external "@mui/utils/useTimeout" ***!
  \****************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@mui/utils/useTimeout");;

/***/ }),

/***/ "@tanstack/react-query":
/*!****************************************!*\
  !*** external "@tanstack/react-query" ***!
  \****************************************/
/***/ ((module) => {

"use strict";
module.exports = import("@tanstack/react-query");;

/***/ }),

/***/ "clsx":
/*!***********************!*\
  !*** external "clsx" ***!
  \***********************/
/***/ ((module) => {

"use strict";
module.exports = import("clsx");;

/***/ }),

/***/ "next/dist/compiled/next-server/pages.runtime.dev.js":
/*!**********************************************************************!*\
  !*** external "next/dist/compiled/next-server/pages.runtime.dev.js" ***!
  \**********************************************************************/
/***/ ((module) => {

"use strict";
module.exports = require("next/dist/compiled/next-server/pages.runtime.dev.js");

/***/ }),

/***/ "prop-types":
/*!*****************************!*\
  !*** external "prop-types" ***!
  \*****************************/
/***/ ((module) => {

"use strict";
module.exports = require("prop-types");

/***/ }),

/***/ "react":
/*!************************!*\
  !*** external "react" ***!
  \************************/
/***/ ((module) => {

"use strict";
module.exports = require("react");

/***/ }),

/***/ "react-transition-group":
/*!*****************************************!*\
  !*** external "react-transition-group" ***!
  \*****************************************/
/***/ ((module) => {

"use strict";
module.exports = require("react-transition-group");

/***/ }),

/***/ "react/jsx-dev-runtime":
/*!****************************************!*\
  !*** external "react/jsx-dev-runtime" ***!
  \****************************************/
/***/ ((module) => {

"use strict";
module.exports = require("react/jsx-dev-runtime");

/***/ }),

/***/ "react/jsx-runtime":
/*!************************************!*\
  !*** external "react/jsx-runtime" ***!
  \************************************/
/***/ ((module) => {

"use strict";
module.exports = require("react/jsx-runtime");

/***/ })

};
;

// load runtime
var __webpack_require__ = require("../webpack-runtime.js");
__webpack_require__.C(exports);
var __webpack_exec__ = (moduleId) => (__webpack_require__(__webpack_require__.s = moduleId))
var __webpack_exports__ = __webpack_require__.X(0, ["vendor-chunks/next","vendor-chunks/@swc","vendor-chunks/@mui"], () => (__webpack_exec__("(pages-dir-node)/./src/pages/_app.tsx")));
module.exports = __webpack_exports__;

})();