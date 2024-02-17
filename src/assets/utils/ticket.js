/**
 * Generate a random string.
 * @return {string} The random generated ticket.
 */
export function createTicket() {
    const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let ticket = "";

    for (let i = 0; i < 8; i++) {
        const randomIndex = Math.floor(Math.random() * characters.length);
        ticket += characters.charAt(randomIndex);
    }

    return ticket;
}
