#include<iostream>
#include<glad/glad.h>
#include<GLFW/glfw3.h>

// Vertex Shader source code
const char* vertexShaderSource = "#version 330 core\n"
"layout (location = 0) in vec3 aPos;\n"
"void main()\n"
"{\n"
"   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n"
"}\0";
//Fragment Shader source code
const char* fragmentShaderSource = "#version 330 core\n"
"out vec4 FragColor;\n"
"void main()\n"
"{\n"
"   FragColor = vec4(0.8f, 0.3f, 0.02f, 1.0f);\n"
"}\n\0";

int main()
{
	// Initialize GLFW
	glfwInit();



	// Tell GLFW what version of OpenGL we are using 
	// In this case we are using OpenGL 3.3
	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
	// Tell GLFW we are using the CORE profile
	// So that means we only have the modern functions
	glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

	// Vertices coordinates
	GLfloat vertices[] =
	{
		-0.5f, -0.5f * float(sqrt(3)) / 3, 0.0f, // Lower left corner
		0.5f, -0.5f * float(sqrt(3)) / 3, 0.0f, // Lower right corner
		0.0f, 0.5f * float(sqrt(3)) * 2 / 3, 0.0f, // Upper corner
		-0.5f / 2, 0.5f * float(sqrt(3)) / 6, 0.0f, // Inner left
		0.5f / 2, 0.5f * float(sqrt(3)) / 6, 0.0f, // Inner right
		0.0f, -0.5f * float(sqrt(3)) / 3, 0.0f // Inner down
	};

	GLuint indicies[] =
	{
		0, 3, 5,
		3, 2, 4,
		5, 4, 1
	};

	//Square verticies coordinate and indicies
	GLfloat s_vertices[] = {
		-0.5f, -0.5f, 0.0f, // bottom left corner
		-0.5f, 0.5f, 0.0f, // top left corner
		0.5f, 0.5f, 0.0f, // top right corner
		0.5f, -0.5f, 0.0f, // bottom right corner
	};

	GLuint s_indicies[] = {
		0, 1, 2,
		0, 2, 3
	};


	// Create a GLFWwindow object of 800 by 800 pixels, naming it "YoutubeOpenGL"
	GLFWwindow* window = glfwCreateWindow(800, 800, "Triforce in window", NULL, NULL);
	// Error check if the window fails to create
	if (window == NULL)
	{
		std::cout << "Failed to create GLFW window" << std::endl;
		glfwTerminate();
		return -1;
	}
	// Introduce the window into the current context
	glfwMakeContextCurrent(window);



	//Load GLAD so it configures OpenGL
	gladLoadGL();
	// Specify the viewport of OpenGL in the Window
	// In this case the viewport goes from x = 0, y = 0, to x = 800, y = 800
	glViewport(0, 0, 800, 800);

	//Create the vertex shader object
	GLuint vertexShader = glCreateShader(GL_VERTEX_SHADER);
	//load the source code
	glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
	//compile source code into machine code for GPU
	glCompileShader(vertexShader);

	//same as above but for fragments
	GLuint fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
	glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
	glCompileShader(fragmentShader);

	// Create the shader program
	GLuint shaderProgram = glCreateProgram();
	//Attach all references to the program
	glAttachShader(shaderProgram, vertexShader);
	glAttachShader(shaderProgram, fragmentShader);
	//Link all the references together
	glLinkProgram(shaderProgram);

	//Since loaded into the program already can delete from memory
	glDeleteShader(vertexShader);
	glDeleteShader(fragmentShader);


	//Vertex Array Object & Vertex Buffer Object reference containers
	GLuint VAOs[1], VBOs[1], EBO;

	//Only 1 object in each
	glGenVertexArrays(1, VAOs);
	glGenBuffers(1, VBOs);
	glGenBuffers(1, &EBO);

	//Make VAO the currect vertex array object
	glBindVertexArray(VAOs[0]);

	//Bind the VBO
	glBindBuffer(GL_ARRAY_BUFFER, VBOs[0]);
	// Introduce the known verticies into the VBO
	glBufferData(GL_ARRAY_BUFFER, sizeof(s_vertices), s_vertices, GL_STATIC_DRAW);

	glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, EBO);
	glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(s_indicies), s_indicies, GL_STATIC_DRAW);

	//configure the VAO so that opengl knows how to interpret the VBO
	glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
	//Enable the VAO so that opengl can use it
	glEnableVertexAttribArray(0);

	//Optional step but nice
	//bind both the VBO and VAO to 0 so that they don't accidentaly modify them
	glBindBuffer(GL_ARRAY_BUFFER, 0);
	glBindVertexArray(0);
	glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);



	// Specify the color of the background
	glClearColor(0.07f, 0.13f, 0.17f, 1.0f);
	// Clean the back buffer and assign the new color to it
	glClear(GL_COLOR_BUFFER_BIT);
	// Swap the back buffer with the front buffer
	glfwSwapBuffers(window);



	// Main while loop
	while (!glfwWindowShouldClose(window))
	{

		glClearColor(0.07f, 0.13f, 0.17f, 1.0f);
		glClear(GL_COLOR_BUFFER_BIT);
		glUseProgram(shaderProgram);
		glBindVertexArray(VAOs[0]);
		glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0);
		glfwSwapBuffers(window);


		// Take care of all GLFW events
		glfwPollEvents();
	}



	// Delete all the objects we've created
	glDeleteVertexArrays(1, VAOs);
	glDeleteBuffers(1, VBOs);
	glDeleteProgram(shaderProgram);
	glDeleteBuffers(1, &EBO);



	// Delete window before ending the program
	glfwDestroyWindow(window);
	// Terminate GLFW before ending the program
	glfwTerminate();
	return 0;
}