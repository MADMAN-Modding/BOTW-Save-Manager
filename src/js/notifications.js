/**Stores how many milliseconds until the notification hides itself.*/
let displayTime = 3000;

/**Tracks the unique ID for each notification to ensure correct behavior when multiple notifications are pushed.*/
let notificationID = 0;

/**
 * Displays a notification with the provided text.
 *
 * @param {string} text - The text to display in the notification.
 *
 * Increments the notification ID to uniquely track the notification.
 *
 * Updates the notification box and text elements to show the provided message.
 *
 * Automatically hides the notification after the specified display time, unless a new notification is pushed before it times out.
 * 
 * @async
 * @returns {void}
 */
async function pushNotification(text) {
    // Increment the notification ID to track this notification.
    notificationID++;

    let pushID = notificationID; // Capture the current notification ID.

    let notificationBox = document.getElementById("notification"); // Get the notification box element.

    let notificationText = document.getElementById("notificationText"); // Get the notification text element.

    notificationText.innerHTML = text; // Set the notification text.

    notificationBox.style.top = "20px"; // Move the notification box into view.

    setTimeout(() => {
        // Check if this notification is still the latest one.
        // If it is hide the notification box.
        if (notificationID == pushID) {
            notificationText.innerHTML = ""; // Clear the notification text.

            notificationBox.style.top = "-80px"; // Move the notification box out of view.
        }
    }, displayTime); // Hide the notification after the specified display time.
}
