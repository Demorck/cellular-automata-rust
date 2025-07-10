use bitflags::bitflags;

bitflags! {
    pub struct Fast30Option: u8 {
        /// Affiche les informations avant et après l'élusion.
        const SAVE_DIAGONALS = 0b0000_0001;
        /// Sauvegarde l'état dans un fichier.
        const LOG_DOUBLING = 0b0000_0010;
        /// Affiche les informations de sauvegarde.
        const LOG_STEPS = 0b0000_0100;
    }
}