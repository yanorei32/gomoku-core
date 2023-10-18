#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::UpRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, 1),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, -1),
        }
    }
}

const DIRECTION_PAIRS: [(Direction, Direction); 4] = [
    (Direction::Up, Direction::Down),
    (Direction::UpRight, Direction::DownLeft),
    (Direction::Right, Direction::Left),
    (Direction::DownRight, Direction::UpLeft),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Coordinate<const X: usize, const Y: usize> {
    pub x: usize,
    pub y: usize,
}

struct CoordinateIterator<const X: usize, const Y: usize> {
    current: Coordinate<X, Y>,
    direction: Direction,
}

impl<const X: usize, const Y: usize> Iterator for CoordinateIterator<X, Y> {
    type Item = Coordinate<X, Y>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.current.apply(self.direction)?;
        Some(self.current)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CreateCoordinateError {
    OutOfRange,
}

impl<const X: usize, const Y: usize> Coordinate<X, Y> {
    #[must_use]
    fn apply(&self, direction: Direction) -> Option<Self> {
        let (x, y) = direction.as_offset();
        Self::try_new(self.x.checked_add_signed(x)?, self.y.checked_add_signed(y)?).ok()
    }

    #[must_use]
    fn iter_direction(&self, direction: Direction) -> CoordinateIterator<X, Y> {
        CoordinateIterator {
            current: *self,
            direction,
        }
    }

    #[must_use]
    pub fn try_new(x: usize, y: usize) -> Result<Self, CreateCoordinateError> {
        if (0..X).contains(&x) && (0..Y).contains(&y) {
            Ok(Self { x, y })
        } else {
            Err(CreateCoordinateError::OutOfRange)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Session<const X: usize, const Y: usize, const N: usize> {
    cells: [[Option<Player>; X]; Y],
    player: Player,
    remain_cells: usize,
}

#[derive(Clone, Debug)]
pub enum PlayState<const X: usize, const Y: usize, const N: usize> {
    Continue(Session<X, Y, N>),
    HasWinner((Player, [[Option<Player>; X]; Y])),
    RemainCellsIsZero([[Option<Player>; X]; Y]),
}

#[derive(Clone, Copy, Debug)]
pub enum PutError {
    StoneIsAlreadyExists,
}

impl<const X: usize, const Y: usize, const N: usize> Default for Session<X, Y, N> {
    fn default() -> Self {
        assert_ne!(X * Y, 0);

        Self {
            cells: [[None; X]; Y],
            player: Player::Black,
            remain_cells: X * Y,
        }
    }
}

impl<const X: usize, const Y: usize, const N: usize> Session<X, Y, N> {
    #[must_use]
    pub fn puttable(&self, c: Coordinate<X, Y>) -> bool {
        self.cells[c.y][c.x].is_none()
    }

    #[must_use]
    pub fn cells(&self) -> [[Option<Player>; X]; Y] {
        self.cells
    }

    #[must_use]
    pub fn player(&self) -> Player {
        self.player
    }

    #[must_use]
    pub fn put(&self, c: Coordinate<X, Y>) -> Result<PlayState<X, Y, N>, PutError> {
        if self.cells[c.y][c.x].is_some() {
            return Err(PutError::StoneIsAlreadyExists);
        }

        let mut cells = self.cells;
        cells[c.y][c.x] = Some(self.player);

        for (d0, d1) in DIRECTION_PAIRS {
            let d0 = c
                .iter_direction(d0)
                .take_while(|c| cells[c.y][c.x] == Some(self.player))
                .count();

            let d1 = c
                .iter_direction(d1)
                .take_while(|c| cells[c.y][c.x] == Some(self.player))
                .count();

            if d0 + d1 + 1 >= N {
                return Ok(PlayState::HasWinner((self.player, cells)));
            }
        }

        let remain_cells = self.remain_cells - 1;

        if remain_cells == 0 {
            return Ok(PlayState::RemainCellsIsZero(cells));
        }

        Ok(PlayState::Continue(Self {
            cells,
            player: !self.player,
            remain_cells,
        }))
    }
}
