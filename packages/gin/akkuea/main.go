package main

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func main() {
	// Create a default gin router
	router := gin.Default()

	// Define a route for GET requests to "/"
	router.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK, "Hello World")
	})

	// Start the server on port 8080
	router.Run(":8080")
}
