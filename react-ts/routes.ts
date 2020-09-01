export const Routes = {
  ANY: '*',
  HOME: '/home',
  DEFAULT: '/',

  FOO: {
    ALL: '/foos',
    NEW: '/foo/new',
    EDIT: (id: string): string => `/foo/${id}/edit`,
    ONE: (id: string): string => `/foo/${id}`,
    BAR: (id: string): string => `/foo/${id}/bar`
  }
}
