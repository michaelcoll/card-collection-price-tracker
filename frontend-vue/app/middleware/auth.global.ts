const isProtectedRoute = createRouteMatcher([
  '/collection(.*)',
  '/trade(.*)',
  '/find(.*)',
  '/prefs(.*)',
]);

export default defineNuxtRouteMiddleware((to) => {
  const { isSignedIn } = useAuth();

  if (!isSignedIn.value && isProtectedRoute(to)) {
    return navigateTo('/sign-in');
  }
});
