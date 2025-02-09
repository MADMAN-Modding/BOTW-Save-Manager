/**
 * Makes all the divs for showing the available saves
 * @returns {void}
 * @async
 */
async function makeListings() {

}

async function newBackup() {
    pushNotification("Creating backup");

    let backupName = document.getElementById("backupName").value;

    invoke("new_backup", {"name": backupName}).catch(error => pushNotification(error));

    pushNotification("Backup Complete");
}
