import { createApp } from 'vue'
import router from '@/routes'
import App from '@/App.vue'
import { createPinia } from 'pinia'
import '@/assets/stylesheets/classes.scss'
import '@/assets/stylesheets/variables.scss'
import '@/assets/stylesheets/global.scss'
import 'floating-vue/dist/style.css'
import FloatingVue from 'floating-vue'
import { initWasm } from './helpers/state'

const pinia = createPinia()

let app = createApp(App)
app.use(router)
app.use(pinia)
app.use(FloatingVue)

const mountedApp = app.mount('#app')
initWasm()
  .then(() => {
    // First, redirect to other landing page if we have that setting
    router.push({ name: 'Home' })

    mountedApp.initialize()
})
  .catch((err) => {
    console.error('Failed to initialize app', err)
    mountedApp.failure(err)
  })
