
#include "main.h"

	pros::Controller master(pros::E_CONTROLLER_MASTER);
	pros::MotorGroup right_mg({11, -12, 15});  
	pros::MotorGroup left_mg({-16, 13, 14});  
	pros::Motor hood(5);
	pros::Motor intake(-6);
	pros::Motor intake2(-7);
	pros::ADIDigitalOut pneumatic = pros::ADIDigitalOut('B'); 
	pros::ADIDigitalOut pneumatic2 = pros::ADIDigitalOut('D');

void initialize() {
	pros::lcd::initialize();
	pros::lcd::set_text(1, "we're cooked");
}

//runs when robot disabled b4 match
void disabled() {}

//runs b4 autonomous but after initialize
void competition_initialize() {}

//runs during autonomous
void autonomous() {
	left_mg.move(80);
	right_mg.move(-80);
	pros::delay(500);
	left_mg.move(0);
	right_mg.move(0);
}

//runs during the driver control period
void opcontrol() { 
	master.set_text(0, 0, "we're cooked");
	
	while (true) {
		
		int dir = master.get_analog(ANALOG_LEFT_X) * -1;
		int turn = master.get_analog(ANALOG_RIGHT_Y) * -1; 
		if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_UP)) {
			pneumatic2.set_value(true); 
		}
		else{
			if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_LEFT)) {
				pneumatic2.set_value(false); 
			}
		}
		if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_DOWN)) {
			pneumatic.set_value(true); 
		}
		else{
			if (master.get_digital_new_press(pros::E_CONTROLLER_DIGITAL_RIGHT)) {
				pneumatic.set_value(false); 
			}
		}
		left_mg.move(dir + turn);
		right_mg.move(dir - turn);
		// if (master.get_digital(pros::E_CONTROLLER_DIGITAL_L2)) {
		// 	hood.move(127);
		// }
		// else {
		// 	if (master.get_digital(pros::E_CONTROLLER_DIGITAL_L1)) {
		// 		hood.move(-127);
		// 	}
		// 	else {
		// 		hood.move(0);	
		// 	}
		// }
		
		if (master.get_digital(pros::E_CONTROLLER_DIGITAL_R2)) {
			intake.move(-127);
			intake2.move(-127);
			hood.move(-127);
		}
		else if (master.get_digital(pros::E_CONTROLLER_DIGITAL_R1)) {
			intake.move(-127);
			intake2.move(-127);
			hood.move(127);
		}
		else {
			intake.move(0);
			intake2.move(0);
			hood.move(0);
		}
		pros::delay(20);
	}
}