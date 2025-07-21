import { LightningElement, track } from 'lwc';

export default class Patient extends LightningElement {
    @track name = 'John Doe';
    @track dob = '1970-01-01';
    @track address = '123 Main St';

    save() {
        // Save the patient information
    }
}
