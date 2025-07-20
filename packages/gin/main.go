package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"

	"gin/api"
	"gin/config"
	"gin/middleware"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
)

func main() {
	// Load environment variables from .env
	_ = godotenv.Load()

	// Initialize database connection
	config.InitDB()
	
	// Ensure database connection is closed on exit
	defer config.CloseDB()

	// Handle graceful shutdown
	go func() {
		c := make(chan os.Signal, 1)
		signal.Notify(c, os.Interrupt, syscall.SIGTERM)
		<-c
		log.Println("Shutting down server...")
		config.CloseDB()
		os.Exit(0)
	}()

	router := gin.Default()
	router.Use(middleware.Logger())

	// Register /ping route
	router.GET("/ping", api.PingHandler)

	// Add a database health check endpoint
	router.GET("/health", api.HealthHandler)

	// Get port from config (env), default to 8080
	port := config.GetPort()
	log.Printf("Starting server on port %s", port)

	if err := router.Run(":" + port); err != nil {
		log.Fatalf("could not start server: %v", err)
	}
}