from math import cos, sin, pi
from turtle import pos
import serial
import json
import time
from datetime import datetime
import numpy as np
from sklearn.linear_model import LinearRegression

### chip is rotated so X is Z axis
### robot forward is Z, robot sideways is Y
ROTATION_AXIS = "Z"
FORWARD_AXIS = "X"

dt = 0
start = time.time()
elapsed = 0

coeff = 0
bias = 0

def integrate_velocity(data, dt):
    acc = data["A"][FORWARD_AXIS] * 9.81 #g's to m/s^2
    return acc * dt

def integrate_position(position, angle, velocity, dt):
    dvel = velocity * dt
    rad = angle * pi / 180.0
    position[0] += cos(rad) * dvel
    position[1] += sin(rad) * dvel
    return position    

ser = serial.Serial('/dev/ttyACM0', baudrate=921600)  # open serial port
ser.baudrate = 921600
print(ser.name)         # check which port was really used

velocity = 0

velocity_measurements = []
time_measurements = []

# get first 3s of data
start = time.time()
while True:

    imu_string = ser.readline().decode('utf8')
    imu_data = {}

    try: 
        imu_data = json.loads(imu_string)
    except:
        print("String not JSON format: %s"%imu_string)
        elapsed = 0

    if "G" in imu_data:
        dt = time.time() - start # time since last measurement
        elapsed += dt
        start = time.time()
        velocity += integrate_velocity(imu_data, dt)
        print(velocity)
        velocity_measurements.append(velocity)
        time_measurements.append(elapsed)

    if elapsed > 3.0:
        break
        
# get line of best fit
# linear regression blah blah blah
time_measurements = np.array(time_measurements, dtype=np.float32).reshape((-1, 1)) # X
velocity_measurements = np.array(velocity_measurements, dtype=np.float32) # Y
model = LinearRegression().fit(time_measurements, velocity_measurements)
r_sq = model.score(time_measurements, velocity_measurements)
bias = model.intercept_
coef = model.coef_[0]

print(r_sq)
print(bias)
print(coef)

# exit()

velocity = 0
elapsed = 0

position = [0,0]

start = time.time()
while True:

    imu_string = ser.readline().decode('utf8')
    imu_data = {}

    try: 
        imu_data = json.loads(imu_string)
    except:
        print("String not JSON format: %s"%imu_string)

    if "G" in imu_data:
        dt = time.time() - start # time since last measurement
        elapsed += dt
        start = time.time()
        velocity += integrate_velocity(imu_data, dt)
        corrected_velocity = velocity - ((elapsed * coef) + bias)

        # ONLY START INTEGRATING POSITION IF MESSAGES ARE COMING IN
        # position = integrate_position(position, angle, corrected_vel, dt)
        # print(position)
        
ser.close()             # close port