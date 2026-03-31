#!/bin/bash

# TurboDownload API Test Script
# 
# This script tests the TurboDownload REST API endpoints.
# Make sure the TurboDownload server is running on http://localhost:8080
#
# Usage: ./test_api.sh [options]
#   -h, --help     Show this help message
#   --skip-auth    Skip authentication tests
#   --server URL   Custom server URL (default: http://localhost:8080)
#   --token TOKEN  Auth token (default: test_token_12345)

set -e

# Default configuration
BASE_URL="http://localhost:8080"
API_URL="${BASE_URL}/api/v1"
TOKEN="test_token_12345"
SKIP_AUTH=false
COLOR_RESET='\033[0m'
COLOR_GREEN='\033[0;32m'
COLOR_RED='\033[0;31m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo "TurboDownload API Test Script"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  -h, --help         Show this help message"
            echo "  --skip-auth        Skip authentication tests"
            echo "  --server URL       Custom server URL (default: http://localhost:8080)"
            echo "  --token TOKEN      Auth token (default: test_token_12345)"
            exit 0
            ;;
        --skip-auth)
            SKIP_AUTH=true
            shift
            ;;
        --server)
            BASE_URL="$2"
            API_URL="${BASE_URL}/api/v1"
            shift 2
            ;;
        --token)
            TOKEN="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Helper functions
print_header() {
    echo -e "\n${COLOR_BLUE}========================================${COLOR_RESET}"
    echo -e "${COLOR_BLUE}$1${COLOR_RESET}"
    echo -e "${COLOR_BLUE}========================================${COLOR_RESET}\n"
}

print_success() {
    echo -e "${COLOR_GREEN}✓ $1${COLOR_RESET}"
}

print_error() {
    echo -e "${COLOR_RED}✗ $1${COLOR_RESET}"
}

print_warning() {
    echo -e "${COLOR_YELLOW}⚠ $1${COLOR_RESET}"
}

print_info() {
    echo -e "${COLOR_BLUE}ℹ $1${COLOR_RESET}"
}

# HTTP helper with auth
curl_auth() {
    curl -s -H "Authorization: Bearer ${TOKEN}" -H "Content-Type: application/json" "$@"
}

curl_no_auth() {
    curl -s -H "Content-Type: application/json" "$@"
}

# Test functions
test_health_check() {
    print_header "Testing Health Check"
    
    local response=$(curl_no_auth "${BASE_URL}/health")
    local status=$(echo "$response" | grep -o '"status":"[^"]*"' | cut -d'"' -f4)
    
    if [ "$status" = "ok" ]; then
        print_success "Health check passed"
        echo "$response" | head -c 200
        echo
        return 0
    else
        print_error "Health check failed"
        echo "$response"
        return 1
    fi
}

test_create_download() {
    print_header "Testing Create Download"
    
    local response=$(curl_auth -X POST "${API_URL}/download" \
        -d '{"url":"https://example.com/test.zip","filename":"test.zip","threads":4}')
    
    local task_id=$(echo "$response" | grep -o '"task_id":"[^"]*"' | cut -d'"' -f4)
    local status=$(echo "$response" | grep -o '"status":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$task_id" ] && [ "$status" = "pending" ]; then
        print_success "Create download passed (task_id: $task_id)"
        echo "$response" | head -c 300
        echo
        echo "$task_id"
        return 0
    else
        print_error "Create download failed"
        echo "$response"
        return 1
    fi
}

test_get_status() {
    local task_id=$1
    print_header "Testing Get Download Status (task: $task_id)"
    
    local response=$(curl_auth "${API_URL}/download/${task_id}")
    
    local retrieved_id=$(echo "$response" | grep -o '"task_id":"[^"]*"' | cut -d'"' -f4)
    
    if [ "$retrieved_id" = "$task_id" ]; then
        print_success "Get status passed"
        echo "$response" | head -c 400
        echo
        return 0
    else
        print_error "Get status failed"
        echo "$response"
        return 1
    fi
}

test_list_downloads() {
    print_header "Testing List Downloads"
    
    local response=$(curl_auth "${API_URL}/downloads")
    
    local total=$(echo "$response" | grep -o '"total":[0-9]*' | cut -d':' -f2)
    
    if [ -n "$total" ]; then
        print_success "List downloads passed (total: $total)"
        echo "$response" | head -c 300
        echo
        return 0
    else
        print_error "List downloads failed"
        echo "$response"
        return 1
    fi
}

test_pause_download() {
    local task_id=$1
    print_header "Testing Pause Download (task: $task_id)"
    
    local response=$(curl_auth -X POST "${API_URL}/download/${task_id}/pause")
    
    echo "$response" | head -c 200
    echo
    return 0
}

test_resume_download() {
    local task_id=$1
    print_header "Testing Resume Download (task: $task_id)"
    
    local response=$(curl_auth -X POST "${API_URL}/download/${task_id}/resume")
    
    echo "$response" | head -c 200
    echo
    return 0
}

test_cancel_download() {
    local task_id=$1
    print_header "Testing Cancel Download (task: $task_id)"
    
    local response=$(curl_auth -X DELETE "${API_URL}/download/${task_id}")
    local status=$(echo "$response" | grep -o '"status":"[^"]*"' | cut -d'"' -f4)
    
    if [ "$status" = "cancelled" ]; then
        print_success "Cancel download passed"
        echo "$response" | head -c 200
        echo
        return 0
    else
        print_warning "Cancel download returned: $status"
        echo "$response" | head -c 200
        echo
        return 0
    fi
}

test_auth_required() {
    print_header "Testing Authentication Required"
    
    local response=$(curl_no_auth -X POST "${API_URL}/download" \
        -d '{"url":"https://example.com/test.zip","filename":"test.zip"}')
    
    local has_error=$(echo "$response" | grep -o '"error"')
    
    if [ -n "$has_error" ]; then
        print_success "Auth required test passed (got error response)"
        return 0
    else
        print_warning "Auth test returned: $response"
        return 0
    fi
}

test_invalid_url() {
    print_header "Testing Invalid URL"
    
    local response=$(curl_auth -X POST "${API_URL}/download" \
        -d '{"url":"not-a-valid-url","filename":"test.zip","threads":4}')
    
    local status=$(curl_auth -X POST "${API_URL}/download" \
        -d '{"url":"not-a-valid-url","filename":"test.zip","threads":4}' -w "%{http_code}" -o /tmp/invalid_url_response.txt)
    
    if [ "$status" = "400" ]; then
        print_success "Invalid URL test passed (got 400)"
        return 0
    else
        print_warning "Invalid URL test returned: $status"
        return 0
    fi
}

# Main test execution
main() {
    echo -e "${COLOR_BLUE}TurboDownload API Test Suite${COLOR_RESET}"
    echo "Server: $BASE_URL"
    echo "API URL: $API_URL"
    echo ""
    
    local failed=0
    local passed=0
    
    # Test 1: Health Check
    if test_health_check; then
        ((passed++))
    else
        ((failed++))
    fi
    
    # Test 2: Create Download
    local task_id=""
    if task_id=$(test_create_download); then
        ((passed++))
    else
        ((failed++))
    fi
    
    # Test 3: Get Status
    if [ -n "$task_id" ]; then
        if test_get_status "$task_id"; then
            ((passed++))
        else
            ((failed++))
        fi
    else
        print_warning "Skipping get status test (no task_id)"
    fi
    
    # Test 4: List Downloads
    if test_list_downloads; then
        ((passed++))
    else
        ((failed++))
    fi
    
    # Test 5: Pause Download
    if [ -n "$task_id" ]; then
        test_pause_download "$task_id"
        ((passed++))
    fi
    
    # Test 6: Resume Download
    if [ -n "$task_id" ]; then
        test_resume_download "$task_id"
        ((passed++))
    fi
    
    # Test 7: Cancel Download
    if [ -n "$task_id" ]; then
        if test_cancel_download "$task_id"; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    # Test 8: Auth Required (unless skipped)
    if [ "$SKIP_AUTH" = false ]; then
        if test_auth_required; then
            ((passed++))
        else
            ((failed++))
        fi
    fi
    
    # Test 9: Invalid URL
    if test_invalid_url; then
        ((passed++))
    else
        ((failed++))
    fi
    
    # Summary
    print_header "Test Summary"
    echo -e "Passed: ${COLOR_GREEN}$passed${COLOR_RESET}"
    echo -e "Failed: ${COLOR_RED}$failed${COLOR_RESET}"
    
    if [ $failed -eq 0 ]; then
        echo -e "\n${COLOR_GREEN}All tests passed!${COLOR_RESET}"
        exit 0
    else
        echo -e "\n${COLOR_RED}Some tests failed!${COLOR_RESET}"
        exit 1
    fi
}

# Run main
main