import { Component, inject } from '@angular/core';
import { RouterLink, RouterLinkActive } from '@angular/router';
import { MatIconModule } from '@angular/material/icon';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { AuthService } from '../core/auth.service';
import { ThemeService } from '../core/theme.service';
import { LoginDialogComponent } from '../core/login-dialog/login-dialog.component';

interface NavItem {
  label: string;
  route: string;
  icon: string;
}

@Component({
  selector: 'app-navbar',
  standalone: true,
  imports: [RouterLink, RouterLinkActive, MatIconModule, MatDialogModule],
  templateUrl: './navbar.component.html',
})
export class NavbarComponent {
  protected auth = inject(AuthService);
  protected theme = inject(ThemeService);
  private dialog = inject(MatDialog);

  protected navItems: NavItem[] = [
    { label: 'Collection', route: '/collection', icon: 'grid_view' },
    { label: 'Transactions', route: '/transactions', icon: 'compare_arrows' },
  ];

  onToggleTheme(): void {
    this.theme.toggle();
  }

  onLogin(): void {
    this.dialog.open(LoginDialogComponent, {
      panelClass: 'clerk-login-panel',
      maxWidth: '90vw',
      // Pas de largeur fixe — le composant Clerk gère sa propre largeur max
    });
  }

  onLogout(): void {
    this.auth.logout();
  }
}
