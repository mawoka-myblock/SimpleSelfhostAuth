export interface PrivateUser {
    id: string
    username: string
    profile_pic: string
    email: string
    created_at: string
    admin: boolean
    scopes: string[]
}

export enum AvailableAdminScreens {
    home = "home",
    createUser = "createUser",
    viewUser = "viewUser"
}