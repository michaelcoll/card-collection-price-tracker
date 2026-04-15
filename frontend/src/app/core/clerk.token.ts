import { InjectionToken } from '@angular/core';
import { Clerk } from '@clerk/clerk-js';

export const CLERK = new InjectionToken<InstanceType<typeof Clerk>>('Clerk instance');
