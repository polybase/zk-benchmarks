
function ordinalSuffix(i: number) {
    const j = i % 10,
        k = i % 100
    if (j == 1 && k != 11) {
        return i + 'st'
    }
    if (j == 2 && k != 12) {
        return i + 'nd'
    }
    if (j == 3 && k != 13) {
        return i + 'rd'
    }
    return i + 'th'
}

export function formatDate(dateString: string) {
    const months = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December']
    const dateObj = new Date(dateString)

    const day = dateObj.getUTCDate()
    const monthIndex = dateObj.getUTCMonth()
    const year = dateObj.getUTCFullYear()

    return `${ordinalSuffix(day)} ${months[monthIndex]} ${year}`
}

export function timeSinceLastUpdate(dateString: string) {
    const now = +(new Date())
    const past = +(new Date(dateString))

    // Calculate the time difference in milliseconds
    const timeDifference = now - past

    // Calculate time difference in minutes, hours, and days
    const minutes = Math.floor(timeDifference / (1000 * 60))
    const hours = Math.floor(timeDifference / (1000 * 60 * 60))
    const days = Math.floor(timeDifference / (1000 * 60 * 60 * 24))

    if (minutes < 1) {
        return 'Just now'
    }

    if (minutes < 60) {
        return `${minutes} min${minutes === 1 ? '' : 's'} ago`
    }

    if (hours < 24) {
        return `${hours} hour${hours === 1 ? '' : 's'} ago`
    }

    return `${days} day${days === 1 ? '' : 's'} ago`
}