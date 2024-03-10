import { createPinia } from 'pinia'
import App from "@/App.vue";

export const store = createPinia()

// @ts-ignore
export function setupStore(app: App) {
  app.use(store)
}
