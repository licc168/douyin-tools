
export {}

declare module 'vue' {
  export interface GlobalComponents {
    QuickSearch: typeof import('./src/components/QuickSearch.vue')['default']
    Components: typeof import('./src/components/index.vue')['default']
    ElAside: typeof import('element-plus/es')['ElAside']
    ElButton: typeof import('element-plus/es')['ElButton']
    ElConfigProvider: typeof import('element-plus/es')['ElConfigProvider']
    ElContainer: typeof import('element-plus/es')['ElContainer']
    ElDropdown: typeof import('element-plus/es')['ElDropdown']
    ElDropdownItem: typeof import('element-plus/es')['ElDropdownItem']
    ElDropdownMenu: typeof import('element-plus/es')['ElDropdownMenu']
    ElFooter: typeof import('element-plus/es')['ElFooter']
    ElHeader: typeof import('element-plus/es')['ElHeader']
    ElIcon: typeof import('element-plus/es')['ElIcon']
    ElMain: typeof import('element-plus/es')['ElMain']
    ElMenu: typeof import('element-plus/es')['ElMenu']
    ElMenuItem: typeof import('element-plus/es')['ElMenuItem']
    ElMenuItemGroup: typeof import('element-plus/es')['ElMenuItemGroup']
    ElRow: typeof import('element-plus/es')['ElRow']
    ElScrollbar: typeof import('element-plus/es')['ElScrollbar']
    ElSubMenu: typeof import('element-plus/es')['ElSubMenu']
    ElTable: typeof import('element-plus/es')['ElTable']
    ElTableColumn: typeof import('element-plus/es')['ElTableColumn']
    ElMessage: typeof import('element-plus/es')['ElMessage']
  }
}
