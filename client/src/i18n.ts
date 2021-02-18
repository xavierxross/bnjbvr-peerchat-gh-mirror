import { register, init, getLocaleFromNavigator } from 'svelte-i18n';

register('en', () => import('../locales/en.json'));
register('fr', () => import('../locales/fr.json'));

init({
    fallbackLocale: 'fr',
    initialLocale: getLocaleFromNavigator().split('-')[0]
});

export const supportedLanguages = [['en', 'English'], ['fr', 'Français']];
