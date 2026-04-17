import { inject } from '@angular/core';
import { CanActivateFn } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { AuthService } from './auth.service';
import { LoginDialogComponent } from './login-dialog/login-dialog.component';
import { map } from 'rxjs';

export const authGuard: CanActivateFn = () => {
  const auth = inject(AuthService);
  const dialog = inject(MatDialog);

  if (auth.isLoggedIn()) {
    return true;
  }

  const dialogRef = dialog.open(LoginDialogComponent, {
    panelClass: 'clerk-login-panel',
    maxWidth: '90vw',
  });

  return dialogRef.afterClosed().pipe(map((loggedIn: boolean | undefined) => !!loggedIn));
};
