import { defineStore } from 'pinia'

type ThemeStore = {
  themeOptions: string[]
  advancedRendering: boolean
  selectedTheme: string
}

export const useTheming = defineStore('themeStore', {
  state: () : ThemeStore => ({
    themeOptions: ['dark', 'light', 'oled'],
    advancedRendering: true,
    selectedTheme: 'dark',
  }),
  actions: {
    setThemeState(newTheme : string) {
      if (this.themeOptions.includes(newTheme)) this.selectedTheme = newTheme
      else console.warn('Selected theme is not present. Check themeOptions.')

      this.setThemeClass()
    },
    setThemeClass() {
      for (const theme of this.themeOptions) {
        document.getElementsByTagName('html')[0].classList.remove(`${theme}-mode`)
      }
      document.getElementsByTagName('html')[0].classList.add(`${this.selectedTheme}-mode`)
    },
  },
})
