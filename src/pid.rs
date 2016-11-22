#[derive(Debug)]
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    p: f32,
    i: f32,
    d: f32,
    error: f32,
    integrator: f32,
    imax: f32,
    imin: f32,
    derivator: f32,
    setpoint: f32,
    last_time: i64,
}

impl Default for PID {
    fn default() -> PID {
        PID {
            kp: 2.0,
            ki: 0.0,
            kd: 1.0,
            p: 0.0,
            i: 0.0,
            d: 0.0,
            error: 0.0,
            integrator: 0.0,
            imax: 300.0,
            imin: 300.0,
            derivator: 0.0,
            setpoint: 0.0,
            last_time: 0, 
        }
    }
}

impl PID {
    pub fn set_kp(&mut self, kp: f32) -> &mut Self {
        self.kp = kp;
        self
    }

    pub fn set_ki(&mut self, ki: f32) -> &mut Self {
        self.ki = ki;
        self
    }

    pub fn set_kd(&mut self, kd: f32) -> &mut Self {
        self.kd = kd;
        self
    }

    pub fn set_setpoint(&mut self, setpoint: f32) -> &mut Self {
        self.setpoint = setpoint;
        self.integrator = 0.0;
        self.derivator = 0.0;
        self
    }

    pub fn set_integrator(&mut self, integrator: f32) -> &mut Self {
        self.integrator = integrator;
        self
    }

    pub fn set_derivator(&mut self, derivator: f32) -> &mut Self {
        self.derivator = derivator;
        self
    }

    pub fn update(&mut self, current_val: f32) -> f32 {
        self.error = self.setpoint - current_val;

        self.p = self.kp * self.error;
		self.d = self.kd * ( self.error - self.derivator);
		self.derivator = self.error;

		self.integrator = self.integrator + self.error;

		if self.integrator > self.imax {
            self.integrator = self.imax;
        } else if self.integrator < self.imin {
            self.integrator = self.imin;
        }

		self.i = self.i * self.ki;

		self.p + self.i + self.d
    }
}