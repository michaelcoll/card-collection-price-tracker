import { TestBed } from '@angular/core/testing';
import { provideRouter } from '@angular/router';
import { App } from './app';
import { CLERK } from './core/clerk.token';

// Mock de l'instance Clerk pour les tests
const mockClerk = {
  session: null,
  user: null,
  addListener: vi.fn().mockReturnValue(vi.fn()),
  signOut: vi.fn().mockResolvedValue(undefined),
  mountSignIn: vi.fn(),
  unmountSignIn: vi.fn(),
};

// window.matchMedia n'est pas implémenté dans JSDOM (environnement Vitest)
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

describe('App', () => {
  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [App],
      providers: [provideRouter([]), { provide: CLERK, useValue: mockClerk }],
    }).compileComponents();
  });

  it('should create the app', () => {
    const fixture = TestBed.createComponent(App);
    const app = fixture.componentInstance;
    expect(app).toBeTruthy();
  });
});
