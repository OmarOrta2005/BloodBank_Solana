# 🩸 BloodBank Solana - Explicación del Código

Este programa está desarrollado en **Rust** utilizando el framework **Anchor** sobre la blockchain de **Solana**. Su objetivo es gestionar un banco de sangre hospitalario mediante operaciones **CRUD** (Crear, Leer, Actualizar y Eliminar), asegurando trazabilidad y control de unidades sanguíneas.

---

## 🔑 1. Configuración Inicial

use anchor_lang::prelude::*;

// ID del programa (Se genera al hacer build en SolPG)
declare_id!("EcYLociRgSb8JkLra9fKM1p4iD8uLNmsrjATB37TuvcD");

* Se importa la librería principal de Anchor.
* `declare_id!` define la dirección única del programa dentro de la red de Solana.

---

## ⚙️ 2. Módulo del Programa

#[program]
pub mod bloodbank_solana {

Aquí se definen todas las funciones públicas del smart contract.

---

## 🟢 3. Operaciones CRUD

### 3.1 CREATE - Inicializar Banco de Sangre

pub fn inicializar_banco(ctx: Context<CrearBanco>, nombre_hospital: String) -> Result<()>

* Crea una cuenta en la blockchain que representa el banco de sangre.
* Guarda:

  * El propietario (`owner`)
  * Nombre del hospital
  * Inventario vacío

👉 Utiliza un **PDA (Program Derived Address)** para generar una dirección única del banco.

---

### 3.2 CREATE - Registrar Unidad de Sangre

pub fn registrar_unidad(...)

* Registra una nueva donación de sangre.
* Valida que el usuario sea el propietario autorizado:

require!(banco.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

* Guarda:

  * Código del donante
  * Grupo sanguíneo
  * Volumen en mililitros
  * Días de caducidad

👉 Permite mantener control clínico detallado de cada unidad.

---

### 3.3 UPDATE - Editar Unidad

pub fn editar_unidad(...)

* Busca una unidad por el **código del donante**.
* Si la encuentra:

  * Actualiza grupo sanguíneo, volumen y caducidad.
* Si no:

  * Retorna error `UnidadNoEncontrada`.

👉 Útil en escenarios médicos donde la unidad cambia (ej. separación de componentes).

---

### 3.4 DELETE - Eliminar Unidad

pub fn eliminar_unidad(...)

* Busca la unidad dentro del inventario.
* Si existe:

  * Se elimina (por transfusión o descarte).
* Si no:

  * Retorna error.

---

### 3.5 READ - Ver Inventario

pub fn ver_inventario(...)

* Muestra en consola:

  * Nombre del hospital
  * Inventario de unidades disponibles

👉 Enfocado en monitorear el estado crítico del banco de sangre.

---

## 📦 4. Estructuras de Datos

### 🧬 UnidadSangre

pub struct UnidadSangre {
pub codigo_donante: String,
pub grupo_sanguineo: String,
pub volumen_ml: u16,
pub dias_caducidad: u8,
}

Representa una unidad de sangre almacenada.

📌 Restricciones:

* `codigo_donante`: máximo 20 caracteres
* `grupo_sanguineo`: máximo 15 caracteres

---

### 🏥 BancoSangre

pub struct BancoSangre {
pub owner: Pubkey,
pub nombre_hospital: String,
pub inventario: Vec<UnidadSangre>,
}

* Es la cuenta principal almacenada en la blockchain.
* Contiene:

  * Propietario (hospital o administrador)
  * Nombre del hospital
  * Inventario de unidades

📌 Límite:

* Máximo 15 unidades críticas almacenadas

---

## 🔐 5. Contextos (Accounts)

### CrearBanco

#[derive(Accounts)]
pub struct CrearBanco<'info>

* Define las cuentas necesarias para inicializar el banco:

  * `owner`: firmante y pagador
  * `banco`: cuenta que se crea
  * `system_program`: requerido por Solana

📌 Usa:

* `seeds` → generación de PDA
* `bump` → evitar colisiones

---

### GestionarBanco

#[derive(Accounts)]
pub struct GestionarBanco<'info>

* Se utiliza para todas las operaciones CRUD.
* Requiere:

  * `owner` (firmante)
  * `banco` (cuenta mutable)

---

## ⚠️ 6. Manejo de Errores

#[error_code]
pub enum Errores {

Define errores personalizados:

* `NoEresElOwner` → acceso no autorizado
* `UnidadNoEncontrada` → no existe el registro del donante

---

## 🧠 Conclusión

Este programa implementa un sistema de gestión de banco de sangre en blockchain, destacando:

* 🔑 Control de acceso mediante claves criptográficas (`owner`)
* 🩸 Registro detallado de unidades sanguíneas
* ⚡ Uso de PDAs para direcciones únicas y seguras
* 🔄 Operaciones CRUD sobre inventario médico

Representa un caso práctico de cómo aplicar blockchain en el sector salud, garantizando **trazabilidad, integridad y seguridad** en la gestión de recursos críticos como la sangre.
