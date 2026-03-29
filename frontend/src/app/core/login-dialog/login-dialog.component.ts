import { Component, CUSTOM_ELEMENTS_SCHEMA, OnInit, OnDestroy, inject } from '@angular/core';
import { MatDialogRef, MatDialogModule } from '@angular/material/dialog';
import { Hanko } from '@teamhanko/hanko-elements';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-login-dialog',
  standalone: true,
  imports: [MatDialogModule],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  templateUrl: './login-dialog.component.html',
  styleUrl: './login-dialog.component.css',
})
export class LoginDialogComponent implements OnInit, OnDestroy {
  private readonly dialogRef = inject(MatDialogRef<LoginDialogComponent>);

  // Instance locale pour écouter l'événement de fin d'authentification
  private readonly hanko = new Hanko(environment.hankoApiUrl);
  private cleanupFn: (() => void) | undefined;

  ngOnInit(): void {
    // Ferme le dialog dès que l'authentification est complète
    this.cleanupFn = this.hanko.onSessionCreated(() => {
      this.dialogRef.close(true);
    });
  }

  ngOnDestroy(): void {
    this.cleanupFn?.();
  }
}
