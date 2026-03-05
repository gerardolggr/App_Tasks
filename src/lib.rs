use anchor_lang::prelude::*;

declare_id!("Qx5KW7DL1a6qZBCmXjjbRsmAN2SaJykUrmaAsYNn5AV");

#[program]
pub mod tareas {
    use super::*;

    pub fn crear_tablero(
        context: Context<NewTasksAccount>,
        nombre_tablero: String,
    ) -> Result<()> {
        let owner_id = context.accounts.owner.key();

        context.accounts.task_account.set_inner(TasksAccount {
            owner: owner_id,
            nombre_tablero,
            tareas: Vec::new(),
        });

        Ok(())
    }

    pub fn add_task(context: Context<UpdateTask>, titulo: String) -> Result<()> {
        let task_account = &mut context.accounts.task_account;

        task_account.tareas.push(Task {
            titulo,
            completada: false,
        });

        Ok(())
    }

    pub fn delete_task(context: Context<UpdateTask>, index: u8) -> Result<()> {
        let task_account = &mut context.accounts.task_account;
        let index = index as usize;

        require!(index < task_account.tareas.len(), TaskError::TaskNotFound);
        task_account.tareas.remove(index);

        Ok(())
    }

    pub fn read_task(context: Context<UpdateTask>) -> Result<()> {
        msg!("Total de tareas: {}", context.accounts.task_account.tareas.len());
        Ok(())
    }

    pub fn update_task(context: Context<UpdateTask>, index: u8, titulo: String) -> Result<()> {
        let task_account = &mut context.accounts.task_account;
        let index = index as usize;

        require!(index < task_account.tareas.len(), TaskError::TaskNotFound);
        task_account.tareas[index].titulo = titulo;

        Ok(())
    }
}

#[error_code]
pub enum TaskError {
    #[msg("La tarea no existe")]
    TaskNotFound,
}

// Cuenta principal por usuario que almacena sus tareas
#[account]
#[derive(InitSpace)]
pub struct TasksAccount {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre_tablero: String,

    #[max_len(10)]
    pub tareas: Vec<Task>,
}

// Estructura que almacena las tareas
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Task {
    #[max_len(60)]
    pub titulo: String,

    pub completada: bool,
}

// Define lo que necesita una cuenta para ejecutar la instrucción
#[derive(Accounts)]
pub struct NewTasksAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + TasksAccount::INIT_SPACE,
        seeds = [b"Tareas", owner.key().as_ref()],
        bump
    )]
    pub task_account: Account<'info, TasksAccount>,

    pub system_program: Program<'info, System>,
}

// Estructura que ayuda a dar las instrucciones (CRUD) de la cuenta ya creada
#[derive(Accounts)]
pub struct UpdateTask<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
        seeds = [b"Tareas", owner.key().as_ref()],
        bump
    )]
    pub task_account: Account<'info, TasksAccount>,
}
