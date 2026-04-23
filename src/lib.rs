use anchor_lang::prelude::*;

// ID del programa (Se genera al hacer build en SolPG)
declare_id!("FAn25Q1ZdQM7qjF3udJZKob2oS3Hda8edi1jFuoentJ5");

#[program]
pub mod bloodbank_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa el refrigerador/banco del hospital
    pub fn inicializar_banco(ctx: Context<CrearBanco>, nombre_hospital: String) -> Result<()> {
        let banco = &mut ctx.accounts.banco;
        banco.owner = ctx.accounts.owner.key();
        banco.nombre_hospital = nombre_hospital;
        banco.inventario = Vec::new();
        
        msg!("Banco de sangre inicializado para: {}", banco.nombre_hospital);
        Ok(())
    }

    // 2. CREATE (Dato): Registra una donación exigiendo TODAS las métricas clínicas
    pub fn registrar_unidad(
        ctx: Context<GestionarBanco>, 
        codigo_donante: String, 
        grupo_sanguineo: String, 
        volumen: u16, 
        caducidad: u8
    ) -> Result<()> {
        let banco = &mut ctx.accounts.banco;
        require!(banco.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nueva_unidad = UnidadSangre {
            codigo_donante: codigo_donante.clone(),
            grupo_sanguineo,
            volumen_ml: volumen,
            dias_caducidad: caducidad,
        };

        banco.inventario.push(nueva_unidad);
        msg!("Unidad médica de donante '{}' asegurada en inventario.", codigo_donante);
        Ok(())
    }

    // 3. UPDATE: Modifica parámetros (ej. si se extrae un subproducto y cambia el volumen)
    pub fn editar_unidad(
        ctx: Context<GestionarBanco>, 
        codigo: String, 
        nuevo_grupo: String, 
        nuevo_volumen: u16, 
        nueva_caducidad: u8
    ) -> Result<()> {
        let banco = &mut ctx.accounts.banco;
        require!(banco.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut banco.inventario;
        for i in 0..lista.len() {
            if lista[i].codigo_donante == codigo {
                lista[i].grupo_sanguineo = nuevo_grupo;
                lista[i].volumen_ml = nuevo_volumen;
                lista[i].dias_caducidad = nueva_caducidad;
                msg!("Métricas de la unidad '{}' actualizadas.", codigo);
                return Ok(());
            }
        }
        Err(Errores::UnidadNoEncontrada.into())
    }

    // 4. DELETE: Elimina la unidad cuando es transfundida a un paciente o desechada
    pub fn eliminar_unidad(ctx: Context<GestionarBanco>, codigo: String) -> Result<()> {
        let banco = &mut ctx.accounts.banco;
        require!(banco.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut banco.inventario;
        let index = lista.iter().position(|u| u.codigo_donante == codigo);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Unidad de sangre '{}' procesada y retirada del inventario.", codigo);
            Ok(())
        } else {
            Err(Errores::UnidadNoEncontrada.into())
        }
    }

    // 5. READ: Emite el estado crítico del refrigerador
    pub fn ver_inventario(ctx: Context<GestionarBanco>) -> Result<()> {
        msg!("Hospital: {}", ctx.accounts.banco.nombre_hospital);
        msg!("Inventario Hemático Activo: {:#?}", ctx.accounts.banco.inventario);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct UnidadSangre {
    #[max_len(20)]
    pub codigo_donante: String,
    #[max_len(15)]
    pub grupo_sanguineo: String,
    pub volumen_ml: u16,
    pub dias_caducidad: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BancoSangre {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_hospital: String,
    #[max_len(15)] // Capacidad para 15 unidades críticas en la PDA
    pub inventario: Vec<UnidadSangre>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearBanco<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + BancoSangre::INIT_SPACE,
        seeds = [b"bancosalud", owner.key().as_ref()],
        bump
    )]
    pub banco: Account<'info, BancoSangre>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarBanco<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub banco: Account<'info, BancoSangre>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("Fallo de seguridad: Llave criptográfica no autorizada por el hospital.")]
    NoEresElOwner,
    #[msg("Error médico: El código de donante no coincide con el inventario físico.")]
    UnidadNoEncontrada,
}
