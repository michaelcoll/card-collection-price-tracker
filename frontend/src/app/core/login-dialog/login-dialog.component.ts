import { Component, ElementRef, OnDestroy, OnInit, ViewChild, inject } from '@angular/core';
import { MatDialogRef, MatDialogModule } from '@angular/material/dialog';
import type { Resources } from '@clerk/shared/types';
import { CLERK } from '../clerk.token';

@Component({
  selector: 'app-login-dialog',
  standalone: true,
  imports: [MatDialogModule],
  templateUrl: './login-dialog.component.html',
  styleUrl: './login-dialog.component.css',
})
export class LoginDialogComponent implements OnInit, OnDestroy {
  private readonly dialogRef = inject(MatDialogRef<LoginDialogComponent>);
  private readonly clerk = inject(CLERK);
  private listenerCleanup?: () => void;

  @ViewChild('signInContainer', { static: true })
  signInContainer!: ElementRef<HTMLDivElement>;

  ngOnInit(): void {
    this.clerk.mountSignIn(this.signInContainer.nativeElement);

    // Ferme le dialog dès que la session est créée
    this.listenerCleanup = this.clerk.addListener(({ session }: Resources) => {
      if (session) {
        this.dialogRef.close(true);
      }
    });
  }

  ngOnDestroy(): void {
    this.listenerCleanup?.();
    this.clerk.unmountSignIn(this.signInContainer.nativeElement);
  }
}
