import tensorflow as tf

tf.compat.v1.logging.set_verbosity(tf.compat.v1.logging.ERROR)


class Model:
    num_maps = 1
    num_outputs = 4

    def create(self, width, height):
        self.model = tf.keras.Sequential()
        self.model.add(tf.keras.layers.Input(
            shape=(self.num_maps, width, height)))
        self.model.add(tf.keras.layers.Flatten())
        self.model.add(tf.keras.layers.Dense(
            50, kernel_initializer='zeros'))
        self.model.add(tf.keras.layers.Dense(
            self.num_outputs, kernel_initializer='zeros'))
        self.model.compile(loss='mean_squared_error', optimizer='rmsprop',
                           metrics=['accuracy'])

    def height(self):
        return self.model.inputs[0].shape[3].value

    def width(self):
        return self.model.inputs[0].shape[2].value

    def load(self, dir):
        self.model = tf.keras.models.load_model(dir)

    def save(self, dir):
        # print(self.model.summary())
        # print("Input name:  " + self.model.input.op.name)
        # print("Output name: " + self.model.output.op.name)
        tf.compat.v1.keras.utils.plot_model(
            self.model, to_file='model.png', show_shapes=True)
        tf.saved_model.save(self.model, dir)

    def train(self, input, output):
        es = tf.keras.callbacks.EarlyStopping(
            monitor='loss', mode='auto', verbose=1)
        self.model.fit(input, output,
                       epochs=100,
                       verbose=2,
                       callbacks=[es])

    def predict(self, input):
        return self.model.predict(input)
